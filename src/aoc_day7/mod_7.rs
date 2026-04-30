use std::collections::HashSet;
use std::collections::HashMap;

pub fn run() {
    let input = std::fs::read_to_string("src/aoc_day7/input_day7.txt")
        .expect("Cannot read input_day7.txt");
    println!("Day 7 p1 = {}", p2(&input));
}

pub fn p1(input: &str) -> usize {
    // Parse to a grid of chars; keep all non-empty lines verbatim.
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let height = grid.len();
    if height == 0 {
        return 0;
    }
    let width = grid.iter().map(|r| r.len()).max().unwrap_or(0);

    // Locate S — the single beam source.
    let mut state: HashSet<usize> = HashSet::new();
    let mut start_row = 0usize;
    'find_s: for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                state.insert(c);
                start_row = r;
                break 'find_s;
            }
        }
    }

    // Sweep downward. At each row, every active beam either:
    //   - passes through unchanged, or
    //   - hits a splitter, in which case we count one split and replace
    //     that beam with two beams at cols c-1 and c+1.
    // Using a HashSet automatically merges beams that converge into the
    // same column on the next row.
    let mut total = 0usize;
    for r in (start_row + 1)..height {
        let row = &grid[r];
        let mut next: HashSet<usize> = HashSet::with_capacity(state.len() * 2);

        for &c in &state {
            // Treat positions past this row's length as empty space.
            let cell = row.get(c).copied().unwrap_or('.');
            if cell == '^' {
                total += 1;
                if c > 0 {
                    next.insert(c - 1);
                }
                if c + 1 < width {
                    next.insert(c + 1);
                }
            } else {
                next.insert(c);
            }
        }
        state = next;
    }

    total
}

pub fn p2(input: &str) -> u64{
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let height = grid.len();
    if height == 0 {
        return 0;
    }

    let width = grid.iter().map(|r| r.len()).max().unwrap_or(0);

    // Trouve S - la source unique
    let mut state: HashMap<usize, u64> = HashMap::new();
    let mut start_row = 0usize;
    'find_s: for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                state.insert(c, 1);
                start_row = r;
                break 'find_s;
            }
        }
    }

    // Balayage vers le bas. Même logique que p1 mais on suit le NOMBRE de
    // timelines qui arrivent à chaque colonne, pas juste leur présence.
    // Splitter touché par n timelines → c-1 reçoit n, c+1 reçoit n.
    // Convergences = SOMME (pas fusion).
    for r in (start_row + 1)..height {
        let row = &grid[r];
        let mut next: HashMap<usize, u64> = HashMap::with_capacity(state.len() * 2);
        for (&c, &n) in &state {
            let cell = row.get(c).copied().unwrap_or('.');
            if cell == '^' {
                if c > 0 {
                    *next.entry(c - 1).or_insert(0) += n;
                }
                if c + 1 < width {
                    *next.entry(c + 1).or_insert(0) += n;
                }
            } else {
                *next.entry(c).or_insert(0) += n;
            }
        }
        state = next;
    }

    // Total des timelines qui sortent du manifold.
    state.values().sum()
}



#[cfg(test)]
mod tests {
    use super::p1;

    const EXAMPLE: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn p1_example() {
        assert_eq!(p1(EXAMPLE), 21);
    }
}
