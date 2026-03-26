use std::collections::{HashMap, HashSet};

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == 'S' {
                return (r, c);
            }
        }
    }

    panic!("Aucun point de départ 'S' trouvé");
}

pub fn p1(input: &str) -> usize {
    let grid = parse_grid(input);

    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let (start_row, start_col) = find_start(&grid);

    let mut active: HashSet<usize> = HashSet::new();
    active.insert(start_col);

    let mut splits = 0usize;

    for row in grid.iter().skip(start_row + 1).take(rows - start_row - 1) {
        let mut next_active: HashSet<usize> = HashSet::new();

        for &col in &active {
            if col >= cols {
                continue;
            }

            match row[col] {
                '.' | 'S' => {
                    next_active.insert(col);
                }
                '^' => {
                    splits += 1;

                    if col > 0 {
                        next_active.insert(col - 1);
                    }
                    if col + 1 < cols {
                        next_active.insert(col + 1);
                    }
                }
                other => panic!("Caractère invalide dans la grille: {:?}", other),
            }
        }

        active = next_active;

        if active.is_empty() {
            break;
        }
    }

    splits
}

pub fn p2(input: &str) -> u128 {
    let grid = parse_grid(input);

    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let (start_row, start_col) = find_start(&grid);

    let mut active: HashMap<usize, u128> = HashMap::new();
    active.insert(start_col, 1);

    for row in grid.iter().skip(start_row + 1).take(rows - start_row - 1) {
        let mut next_active: HashMap<usize, u128> = HashMap::new();

        for (&col, &count) in &active {
            if col >= cols {
                continue;
            }

            match row[col] {
                '.' | 'S' => {
                    *next_active.entry(col).or_insert(0) += count;
                }
                '^' => {
                    if col > 0 {
                        *next_active.entry(col - 1).or_insert(0) += count;
                    }
                    if col + 1 < cols {
                        *next_active.entry(col + 1).or_insert(0) += count;
                    }
                }
                other => panic!("Caractère invalide dans la grille: {:?}", other),
            }
        }

        active = next_active;

        if active.is_empty() {
            break;
        }
    }

    active.values().sum()
}

#[cfg(test)]
mod test {
    use super::{p1, p2};

    #[test]
    fn p1_test() {
        let input = "\
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

        assert_eq!(p1(input), 21);
    }

    #[test]
    fn p2_test() {
        let input = "\
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

        assert_eq!(p2(input), 40);
    }
}