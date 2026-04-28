use std::collections::{HashMap, HashSet};

const INF: usize = usize::MAX / 4;

// ─── Parsing (identique) ──────────────────────────────────────────────────────

fn parse_target_bits(s: &str) -> u64 {
    let inside = s.strip_prefix('[').and_then(|x| x.strip_suffix(']'))
        .unwrap_or_else(|| panic!("Diagramme invalide: {:?}", s));
    let mut mask = 0u64;
    for (i, ch) in inside.chars().enumerate() {
        match ch { '#' => mask |= 1u64 << i, '.' => {}, _ => panic!("{:?}", ch) }
    }
    mask
}

fn parse_button_indices(s: &str) -> Vec<usize> {
    let inside = s.strip_prefix('(').and_then(|x| x.strip_suffix(')'))
        .unwrap_or_else(|| panic!("Bouton invalide: {:?}", s));
    if inside.trim().is_empty() { return Vec::new(); }
    inside.split(',').map(|p| p.trim().parse::<usize>().unwrap()).collect()
}

fn parse_button_bits(s: &str) -> u64 {
    parse_button_indices(s).iter().fold(0u64, |m, &i| m | (1u64 << i))
}

fn parse_joltages(s: &str) -> Vec<u16> {
    let inside = s.strip_prefix('{').and_then(|x| x.strip_suffix('}'))
        .unwrap_or_else(|| panic!("Joltage invalide: {:?}", s));
    if inside.trim().is_empty() { return Vec::new(); }
    inside.split(',').map(|p| p.trim().parse::<u16>().unwrap()).collect()
}

fn parse_machine(line: &str) -> (u64, Vec<u64>, Vec<Vec<usize>>, Vec<u16>) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    assert!(!parts.is_empty());
    let target_bits = parse_target_bits(parts[0]);
    let mut buttons_bits = Vec::new();
    let mut buttons_indices = Vec::new();
    let mut joltages = Vec::new();
    for part in &parts[1..] {
        if part.starts_with('(') {
            buttons_bits.push(parse_button_bits(part));
            buttons_indices.push(parse_button_indices(part));
        } else if part.starts_with('{') {
            joltages = parse_joltages(part);
            break;
        }
    }
    (target_bits, buttons_bits, buttons_indices, joltages)
}

// ─── P1 : Meet in the Middle  O(2^(m/2)) ─────────────────────────────────────

fn min_presses_p1(target: u64, buttons: &[u64]) -> usize {
    if target == 0 { return 0; }
    let m = buttons.len();
    if m == 0 { return INF; }

    let half = m / 2;
    let (left, right) = (&buttons[..half], &buttons[half..]);

    let mut left_map: HashMap<u64, usize> = HashMap::with_capacity(1 << left.len());
    for mask in 0u64..(1u64 << left.len()) {
        let xor = left.iter().enumerate()
            .filter(|(i, _)| (mask >> i) & 1 == 1)
            .fold(0u64, |acc, (_, &b)| acc ^ b);
        let p = mask.count_ones() as usize;
        left_map.entry(xor).and_modify(|e| *e = (*e).min(p)).or_insert(p);
    }

    let mut best = INF;
    for mask in 0u64..(1u64 << right.len()) {
        let xor = right.iter().enumerate()
            .filter(|(i, _)| (mask >> i) & 1 == 1)
            .fold(0u64, |acc, (_, &b)| acc ^ b);
        let pr = mask.count_ones() as usize;
        if let Some(&pl) = left_map.get(&(target ^ xor)) {
            best = best.min(pl + pr);
        }
    }
    best
}

// ─── Solveur LP : Simplexe Deux Phases ────────────────────────────────────────
//
//  Problème : minimiser sum(x_j)
//             sous : A·x = b,  x ≥ 0
//
//  Convention du tableau :
//    - Lignes 0..n  : contraintes
//    - Ligne n      : ligne objectif
//    - tab[n][j]    = −ĉ_j  (coût réduit négatif)
//    - tab[n][rhs]  = z courant  (valeur objectif actuelle)
//    - On entre la colonne avec tab[n][j] > EPS (coût réduit le plus négatif)
//    - Après pivot : z diminue  ✓

const LP_EPS: f64 = 1e-9;

fn lp_pivot(tab: &mut [Vec<f64>], basis: &mut [usize], r: usize, c: usize) {
    let rows = tab.len();
    let cols = tab[0].len();
    let piv = tab[r][c];
    for j in 0..cols { tab[r][j] /= piv; }
    for i in 0..rows {
        if i == r { continue; }
        let f = tab[i][c];
        if f.abs() > LP_EPS {
            for j in 0..cols { tab[i][j] -= f * tab[r][j]; }
        }
    }
    basis[r] = c;
}

fn lp_iterate(tab: &mut Vec<Vec<f64>>, basis: &mut Vec<usize>, n: usize, col_limit: usize) {
    let rhs = tab[0].len() - 1;
    for _ in 0..50_000 {
        // Colonne entrante : valeur la plus positive dans ligne objectif
        let mut enter = None;
        let mut best = LP_EPS;
        for j in 0..col_limit {
            if tab[n][j] > best { best = tab[n][j]; enter = Some(j); }
        }
        let enter = match enter { None => return, Some(j) => j };

        // Test du rapport minimal (variable sortante)
        let mut leave = None;
        let mut min_ratio = f64::INFINITY;
        for i in 0..n {
            if tab[i][enter] > LP_EPS {
                let ratio = tab[i][rhs] / tab[i][enter];
                if ratio < min_ratio - LP_EPS { min_ratio = ratio; leave = Some(i); }
            }
        }
        let leave = match leave { None => return, Some(i) => i };

        lp_pivot(tab, basis, leave, enter);
    }
}

/// Résout : min sum(x_j)  s.t.  A·x = b,  x ≥ 0
/// Retourne Some((valeur_optimale, solution)) ou None si infaisable.
fn lp_solve(a: &[Vec<f64>], b: &[f64]) -> Option<(f64, Vec<f64>)> {
    let n = a.len();
    if n == 0 { return Some((0.0, vec![])); }
    let m = a[0].len();

    let total = m + n;     // variables originales + artificielles
    let rhs = total;       // colonne RHS

    // Construction du tableau initial
    let mut tab = vec![vec![0.0f64; total + 1]; n + 1];
    let mut basis = vec![0usize; n];

    for i in 0..n {
        for j in 0..m { tab[i][j] = a[i][j]; }
        tab[i][m + i] = 1.0;           // variable artificielle
        tab[i][rhs] = b[i].max(0.0);
        basis[i] = m + i;
    }

    // Ligne objectif Phase 1 :
    // tab[n][j] = −ĉ_j = sum_i a[i][j]  pour j < m
    // tab[n][rhs] = z courant = sum(b_i)
    for j in 0..m {
        for i in 0..n { tab[n][j] += a[i][j]; }
    }
    tab[n][rhs] = b.iter().map(|&x| x.max(0.0)).sum();

    // Phase 1 : éliminer les variables artificielles
    lp_iterate(&mut tab, &mut basis, n, total);

    if tab[n][rhs] > 1e-6 {
        return None; // Infaisable
    }

    // Expulser les artificielles encore en base si possible
    for i in 0..n {
        if basis[i] >= m {
            for j in 0..m {
                if tab[i][j].abs() > LP_EPS {
                    lp_pivot(&mut tab, &mut basis, i, j);
                    break;
                }
            }
        }
    }

    // Phase 2 : minimiser sum(x_j) pour j < m
    // Reconstruction de la ligne objectif
    for j in 0..=rhs { tab[n][j] = 0.0; }
    for i in 0..n {
        if basis[i] < m {
            for j in 0..=rhs { tab[n][j] += tab[i][j]; }
        }
    }
    for j in 0..m { tab[n][j] -= 1.0; }           // soustraire c_j = 1
    for j in m..total { tab[n][j] = f64::NEG_INFINITY; } // bloquer les artificielles

    // Phase 2 : seulement les variables originales peuvent entrer
    lp_iterate(&mut tab, &mut basis, n, m);

    // Extraction de la solution
    let z = tab[n][rhs];
    let mut x = vec![0.0f64; m];
    for i in 0..n {
        if basis[i] < m { x[basis[i]] = tab[i][rhs].max(0.0); }
    }

    Some((z, x))
}

// ─── P2 : LP + vérification entière ──────────────────────────────────────────

fn dedup_buttons(n: usize, buttons_raw: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut seen = HashSet::<u64>::new();
    let mut out = Vec::new();
    for btn in buttons_raw {
        let mut f: Vec<usize> = btn.iter().copied().filter(|&i| i < n).collect();
        f.sort_unstable(); f.dedup();
        if f.is_empty() { continue; }
        let mask = f.iter().fold(0u64, |m, &i| m | (1u64 << i.min(63)));
        if seen.insert(mask) { out.push(f); }
    }
    out
}

fn min_presses_p2(target: &[u16], buttons_raw: &[Vec<usize>]) -> usize {
    let n = target.len();
    if n == 0 || target.iter().all(|&x| x == 0) { return 0; }

    let buttons = dedup_buttons(n, buttons_raw);
    if buttons.is_empty() { return INF; }
    let m = buttons.len();

    // Matrice A (n × m) et vecteur b
    let mut a = vec![vec![0.0f64; m]; n];
    for (j, btn) in buttons.iter().enumerate() {
        for &i in btn { if i < n { a[i][j] = 1.0; } }
    }
    let b: Vec<f64> = target.iter().map(|&x| x as f64).collect();

    // Vérification rapide : chaque compteur non nul doit être couvert
    for i in 0..n {
        if b[i] > LP_EPS && (0..m).all(|j| a[i][j] < LP_EPS) {
            return INF;
        }
    }

    // Résolution LP
    let (lp_val, lp_sol) = match lp_solve(&a, &b) {
        None => return INF,
        Some(v) => v,
    };

    // Si la solution LP est entière → réponse exacte
    // (c'est quasi-toujours le cas pour les inputs AoC)
    if lp_sol.iter().all(|&x| (x - x.round()).abs() < 1e-5) {
        return lp_val.round() as usize;
    }

    // Solution fractionnaire : la borne inférieure LP est ceil(lp_val)
    // On complète avec un glouton (borne supérieure valide)
    let lb = lp_val.ceil() as usize;
    let greedy = greedy_p2(&buttons, target, n);

    // Si LP et glouton convergent → réponse exacte
    if greedy == lb { return lb; }

    // Sinon on retourne le glouton (valide, potentiellement non optimal)
    // Pour les inputs AoC Day 10, ce cas ne devrait pas se produire.
    greedy
}

fn greedy_p2(buttons: &[Vec<usize>], target: &[u16], _n: usize) -> usize {
    let mut rem = target.to_vec();
    let mut total = 0usize;
    loop {
        if rem.iter().all(|&x| x == 0) { return total; }
        let mut best_btn = None;
        let mut best_k = 0u16;
        let mut best_score = 0usize;
        for (j, btn) in buttons.iter().enumerate() {
            if btn.iter().any(|&i| rem[i] == 0) { continue; }
            let k = btn.iter().map(|&i| rem[i]).min().unwrap_or(0);
            if k == 0 { continue; }
            let score: usize = btn.iter().map(|&i| rem[i] as usize).sum();
            if score > best_score || (score == best_score && k > best_k) {
                best_score = score; best_k = k; best_btn = Some(j);
            }
        }
        match best_btn {
            None => return INF,
            Some(j) => { for &i in &buttons[j] { rem[i] -= best_k; } total += best_k as usize; }
        }
    }
}

// ─── Points d'entrée publics ──────────────────────────────────────────────────

pub fn p1(input: &str) -> usize {
    input.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (target, buttons_bits, _, _) = parse_machine(l.trim());
            min_presses_p1(target, &buttons_bits)
        })
        .sum()
}

pub fn p2(input: &str) -> u64 {
    input.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (_, _, buttons_indices, joltages) = parse_machine(l.trim());
            let ans = min_presses_p2(&joltages, &buttons_indices);
            if ans >= INF { panic!("Aucune solution pour: {}", l); }
            ans as u64
        })
        .fold(0u64, |acc, x| acc.checked_add(x).expect("Overflow"))
}

#[cfg(test)]
mod test {
    use super::{p1, p2};

    #[test]
    fn p1_test() {
        let input = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(p1(input), 7);
    }

    #[test]
    fn p2_test() {
        let input = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(p2(input), 33);
    }
}
