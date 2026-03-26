use std::collections::{HashMap, HashSet};

const INF: usize = usize::MAX / 4;
const BITS_PER_COUNTER: u32 = 12;
const COUNTER_MASK: u128 = (1u128 << BITS_PER_COUNTER) - 1;

fn parse_target_bits(s: &str) -> u64 {
    let inside = s
        .strip_prefix('[')
        .and_then(|x| x.strip_suffix(']'))
        .unwrap_or_else(|| panic!("Diagramme invalide: {:?}", s));

    let mut mask = 0u64;

    for (i, ch) in inside.chars().enumerate() {
        match ch {
            '#' => mask |= 1u64 << i,
            '.' => {}
            _ => panic!("Caractère invalide dans le diagramme: {:?}", ch),
        }
    }

    mask
}

fn parse_button_indices(s: &str) -> Vec<usize> {
    let inside = s
        .strip_prefix('(')
        .and_then(|x| x.strip_suffix(')'))
        .unwrap_or_else(|| panic!("Bouton invalide: {:?}", s));

    if inside.trim().is_empty() {
        return Vec::new();
    }

    inside
        .split(',')
        .map(|part| {
            part.trim()
                .parse::<usize>()
                .unwrap_or_else(|e| panic!("Indice invalide dans {:?}: {}", s, e))
        })
        .collect()
}

fn parse_button_bits(s: &str) -> u64 {
    let mut mask = 0u64;
    for idx in parse_button_indices(s) {
        mask |= 1u64 << idx;
    }
    mask
}

fn parse_joltages(s: &str) -> Vec<u16> {
    let inside = s
        .strip_prefix('{')
        .and_then(|x| x.strip_suffix('}'))
        .unwrap_or_else(|| panic!("Bloc joltage invalide: {:?}", s));

    if inside.trim().is_empty() {
        return Vec::new();
    }

    inside
        .split(',')
        .map(|part| {
            part.trim()
                .parse::<u16>()
                .unwrap_or_else(|e| panic!("Valeur joltage invalide dans {:?}: {}", s, e))
        })
        .collect()
}

fn parse_machine(line: &str) -> (u64, Vec<u64>, Vec<Vec<usize>>, Vec<u16>) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.is_empty() {
        panic!("Ligne vide invalide");
    }

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

fn min_presses_for_machine_p1(target: u64, buttons: &[u64]) -> usize {
    let m = buttons.len();
    let mut best = usize::MAX;

    for mask in 0usize..(1usize << m) {
        let presses = mask.count_ones() as usize;
        if presses >= best {
            continue;
        }

        let mut state = 0u64;

        for (i, &button) in buttons.iter().enumerate() {
            if ((mask >> i) & 1) == 1 {
                state ^= button;
            }
        }

        if state == target {
            best = presses;
        }
    }

    best
}

fn pack_state(values: &[u16]) -> u128 {
    let mut packed = 0u128;
    for (i, &v) in values.iter().enumerate() {
        let shift = (i as u32) * BITS_PER_COUNTER;
        packed |= (v as u128) << shift;
    }
    packed
}

fn lower_bound_simple(remaining: &[u16], max_button_size: usize) -> usize {
    let max_need = remaining.iter().copied().max().unwrap_or(0) as usize;
    let sum_need: usize = remaining.iter().map(|&x| x as usize).sum();
    let by_sum = sum_need.div_ceil(max_button_size.max(1));
    max_need.max(by_sum)
}

fn preprocess_buttons(n: usize, buttons_raw: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut seen = HashSet::<u16>::new();
    let mut buttons = Vec::<Vec<usize>>::new();

    for button in buttons_raw {
        let mut filtered: Vec<usize> = button.iter().copied().filter(|&idx| idx < n).collect();
        filtered.sort_unstable();
        filtered.dedup();

        if filtered.is_empty() {
            continue;
        }

        let mut mask = 0u16;
        for &idx in &filtered {
            mask |= 1u16 << idx;
        }

        if seen.insert(mask) {
            buttons.push(filtered);
        }
    }

    buttons
}

fn build_counter_to_buttons(n: usize, buttons: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut counter_to_buttons = vec![Vec::<usize>::new(); n];
    for (bi, button) in buttons.iter().enumerate() {
        for &idx in button {
            counter_to_buttons[idx].push(bi);
        }
    }
    counter_to_buttons
}

fn choose_pivot(
    remaining: &[u16],
    buttons: &[Vec<usize>],
    counter_to_buttons: &[Vec<usize>],
    active_mask: u64,
) -> Option<(usize, Vec<usize>)> {
    let mut best_idx = None;
    let mut best_candidates = Vec::new();
    let mut best_count = usize::MAX;
    let mut best_need = 0u16;

    for i in 0..remaining.len() {
        if remaining[i] == 0 {
            continue;
        }

        let candidates: Vec<usize> = counter_to_buttons[i]
            .iter()
            .copied()
            .filter(|&bi| {
                ((active_mask >> bi) & 1) == 1
                    && buttons[bi].iter().all(|&idx| remaining[idx] > 0)
            })
            .collect();

        if candidates.is_empty() {
            return Some((i, Vec::new()));
        }

        if candidates.len() < best_count
            || (candidates.len() == best_count && remaining[i] > best_need)
        {
            best_count = candidates.len();
            best_need = remaining[i];
            best_candidates = candidates;
            best_idx = Some(i);
        }
    }

    best_idx.map(|idx| (idx, best_candidates))
}

fn greedy_upper_bound(
    mut remaining: Vec<u16>,
    buttons: &[Vec<usize>],
    active_mask: u64,
) -> usize {
    let mut steps = 0usize;

    while remaining.iter().any(|&x| x > 0) {
        let mut best_button: Option<usize> = None;
        let mut best_score = 0usize;
        let mut best_k = 0u16;

        for (bi, button) in buttons.iter().enumerate() {
            if ((active_mask >> bi) & 1) == 0 {
                continue;
            }

            if button.iter().any(|&idx| remaining[idx] == 0) {
                continue;
            }

            let k = button.iter().map(|&idx| remaining[idx]).min().unwrap_or(0);
            if k == 0 {
                continue;
            }

            let score: usize = button.iter().map(|&idx| remaining[idx] as usize).sum();

            if score > best_score || (score == best_score && k > best_k) {
                best_score = score;
                best_k = k;
                best_button = Some(bi);
            }
        }

        let bi = match best_button {
            Some(x) => x,
            None => return INF,
        };

        for &idx in &buttons[bi] {
            remaining[idx] -= best_k;
        }
        steps += best_k as usize;
    }

    steps
}

fn min_presses_for_machine_p2(target: &[u16], buttons_raw: &[Vec<usize>]) -> usize {
    let n = target.len();

    if n == 0 {
        return 0;
    }

    for &v in target {
        assert!(
            (v as u128) <= COUNTER_MASK,
            "Valeur trop grande pour le compactage: {}",
            v
        );
    }

    let buttons = preprocess_buttons(n, buttons_raw);
    if buttons.is_empty() {
        return if target.iter().all(|&x| x == 0) { 0 } else { INF };
    }

    let m = buttons.len();
    assert!(m <= 64, "Trop de boutons pour le masque actif: {}", m);

    let counter_to_buttons = build_counter_to_buttons(n, &buttons);

    for i in 0..n {
        if target[i] > 0 && counter_to_buttons[i].is_empty() {
            return INF;
        }
    }

    let max_button_size = buttons.iter().map(|b| b.len()).max().unwrap_or(1);
    let mut memo = HashMap::<(u128, u64), usize>::new();

    fn solve(
        remaining: &[u16],
        active_mask: u64,
        buttons: &[Vec<usize>],
        counter_to_buttons: &[Vec<usize>],
        max_button_size: usize,
        memo: &mut HashMap<(u128, u64), usize>,
    ) -> usize {
        let key = (pack_state(remaining), active_mask);

        if let Some(&ans) = memo.get(&key) {
            return ans;
        }

        if remaining.iter().all(|&x| x == 0) {
            memo.insert(key, 0);
            return 0;
        }

        let lb = lower_bound_simple(remaining, max_button_size);
        let greedy = greedy_upper_bound(remaining.to_vec(), buttons, active_mask);
        if lb == greedy {
            memo.insert(key, lb);
            return lb;
        }

        let (pivot, candidate_buttons) =
            match choose_pivot(remaining, buttons, counter_to_buttons, active_mask) {
                Some(v) => v,
                None => {
                    memo.insert(key, 0);
                    return 0;
                }
            };

        if candidate_buttons.is_empty() {
            memo.insert(key, INF);
            return INF;
        }

        let pivot_need = remaining[pivot] as usize;

        let mut next_active_mask = active_mask;
        for &bi in &candidate_buttons {
            next_active_mask &= !(1u64 << bi);
        }

        let mut best = INF;
        let mut delta = vec![0u16; remaining.len()];

        fn enumerate_assignments(
            pos: usize,
            remaining_pivot: u16,
            candidate_buttons: &[usize],
            buttons: &[Vec<usize>],
            remaining: &[u16],
            delta: &mut [u16],
            next_active_mask: u64,
            counter_to_buttons: &[Vec<usize>],
            max_button_size: usize,
            memo: &mut HashMap<(u128, u64), usize>,
            best: &mut usize,
            pivot_need: usize,
        ) {
            if pos == candidate_buttons.len() {
                if remaining_pivot != 0 {
                    return;
                }

                let mut next = remaining.to_vec();
                for i in 0..next.len() {
                    if delta[i] > next[i] {
                        return;
                    }
                    next[i] -= delta[i];
                }

                let sub = solve(
                    &next,
                    next_active_mask,
                    buttons,
                    counter_to_buttons,
                    max_button_size,
                    memo,
                );

                if sub < INF {
                    *best = (*best).min(pivot_need + sub);
                }

                return;
            }

            let bi = candidate_buttons[pos];
            let button = &buttons[bi];

            let max_count_by_remaining = button
                .iter()
                .map(|&idx| remaining[idx] - delta[idx])
                .min()
                .unwrap_or(0);

            let max_count = max_count_by_remaining.min(remaining_pivot);

            for count in (0..=max_count).rev() {
                if count > 0 {
                    for &idx in button {
                        delta[idx] += count;
                    }
                }

                enumerate_assignments(
                    pos + 1,
                    remaining_pivot - count,
                    candidate_buttons,
                    buttons,
                    remaining,
                    delta,
                    next_active_mask,
                    counter_to_buttons,
                    max_button_size,
                    memo,
                    best,
                    pivot_need,
                );

                if count > 0 {
                    for &idx in button {
                        delta[idx] -= count;
                    }
                }
            }
        }

        enumerate_assignments(
            0,
            remaining[pivot],
            &candidate_buttons,
            buttons,
            remaining,
            &mut delta,
            next_active_mask,
            counter_to_buttons,
            max_button_size,
            memo,
            &mut best,
            pivot_need,
        );

        memo.insert(key, best);
        best
    }

    solve(
        target,
        if m == 64 { u64::MAX } else { (1u64 << m) - 1 },
        &buttons,
        &counter_to_buttons,
        max_button_size,
        &mut memo,
    )
}

pub fn p1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (target, buttons_bits, _, _) = parse_machine(line.trim());
            min_presses_for_machine_p1(target, &buttons_bits)
        })
        .sum()
}

pub fn p2(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (_, _, buttons_indices, joltages) = parse_machine(line.trim());
            let ans = min_presses_for_machine_p2(&joltages, &buttons_indices);
            if ans >= INF {
                panic!("Aucune solution trouvée pour la machine: {}", line);
            }
            ans as u64
        })
        .fold(0u64, |acc, x| {
            acc.checked_add(x)
                .expect("Overflow total dans p2")
        })
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