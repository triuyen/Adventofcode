fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn count_neighbors(grid: &[Vec<char>], r: usize, c: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut neighbors = 0;

    for (dr, dc) in directions {
        let nr = r as isize + dr;
        let nc = c as isize + dc;

        if nr >= 0
            && nr < rows as isize
            && nc >= 0
            && nc < cols as isize
            && grid[nr as usize][nc as usize] == '@'
        {
            neighbors += 1;
        }
    }

    neighbors
}

fn find_accessible(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '@' && count_neighbors(grid, r, c) < 4 {
                accessible.push((r, c));
            }
        }
    }

    accessible
}

pub fn p1(input: &str) -> usize {
    let grid = parse_grid(input);

    if grid.is_empty() {
        return 0;
    }

    find_accessible(&grid).len()
}

pub fn p2(input: &str) -> usize {
    let mut grid = parse_grid(input);

    if grid.is_empty() {
        return 0;
    }

    let mut removed_total = 0;

    loop {
        let accessible = find_accessible(&grid);

        if accessible.is_empty() {
            break;
        }

        removed_total += accessible.len();

        for (r, c) in accessible {
            grid[r][c] = '.';
        }
    }

    removed_total
}

#[cfg(test)]
mod test {
    use super::{p1, p2};

    #[test]
    fn p1_test() {
        let input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(p1(input), 13);
    }

    #[test]
    fn p2_test() {
        let input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(p2(input), 43);
    }
}