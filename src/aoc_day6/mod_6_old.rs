pub fn run() {
    let input = std::fs::read_to_string("src/aoc_day6/input_day6.txt")
        .expect("Cannot read input_day6.txt");
    println!("Day 6 p1 = {}", p1(&input));
    println!("Day 6 p2 = {}", p2(&input));
}




/// Parse to a uniform grid of chars, padding lines with spaces to the max width.
fn pad_grid(input: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    for row in &mut lines {
        if row.len() < width {
            row.resize(width, ' ');
        }
    }
    lines
}

/// Maximal runs of columns where at least one row is non-space.
/// A column is a "separator" iff every row has a space at that column.
fn problem_ranges(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let width = grid.first().map(|r| r.len()).unwrap_or(0);
    let is_sep = |col: usize| grid.iter().all(|row| row[col] == ' ');

    let mut ranges = Vec::new();
    let mut start = 0usize;
    let mut in_problem = false;
    for col in 0..width {
        if is_sep(col) {
            if in_problem {
                ranges.push((start, col));
                in_problem = false;
            }
        } else if !in_problem {
            start = col;
            in_problem = true;
        }
    }
    if in_problem {
        ranges.push((start, width));
    }
    ranges
}

fn slice_trim(row: &[char], s: usize, e: usize) -> String {
    row[s..e].iter().collect::<String>().trim().to_string()
}

/// Find the operator char ('+' or '*') somewhere in the operator row's slice.
fn find_op(op_row: &[char], s: usize, e: usize) -> char {
    op_row[s..e]
        .iter()
        .copied()
        .find(|&c| c == '+' || c == '*')
        .expect("no operator found in problem range")
}

// ---------------- Part 1 ----------------
// Each ROW within a problem is one number; numbers read horizontally.

pub fn p1(input: &str) -> u128 {
    let grid = pad_grid(input);
    if grid.len() < 2 {
        return 0;
    }
    let (op_row, data_rows) = grid.split_last().unwrap();
    let ranges = problem_ranges(&grid);

    let mut total: u128 = 0;
    for (s, e) in ranges {
        let nums: Vec<u128> = data_rows
            .iter()
            .filter_map(|row| {
                let cell = slice_trim(row, s, e);
                if cell.is_empty() {
                    None
                } else {
                    Some(cell.parse().expect("expected a number"))
                }
            })
            .collect();
        let op = find_op(op_row, s, e);
        total += match op {
            '*' => nums.iter().product::<u128>(),
            '+' => nums.iter().sum::<u128>(),
            _ => unreachable!(),
        };
    }
    total
}

// ---------------- Part 2 ----------------
// Each COLUMN within a problem is one number; digits read top-to-bottom
// (most significant first), spaces skipped. Operator order is irrelevant
// since + and * are commutative.

pub fn p2(input: &str) -> u128 {
    let grid = pad_grid(input);
    if grid.len() < 2 {
        return 0;
    }
    let (op_row, data_rows) = grid.split_last().unwrap();
    let ranges = problem_ranges(&grid);

    let mut total: u128 = 0;
    for (s, e) in ranges {
        let mut nums: Vec<u128> = Vec::new();
        for col in s..e {
            // Concatenate non-space chars from every data row in this column,
            // top-to-bottom = most significant -> least significant.
            let digits: String = data_rows
                .iter()
                .filter_map(|row| {
                    let ch = row[col];
                    if ch == ' ' {
                        None
                    } else {
                        Some(ch)
                    }
                })
                .collect();
            if !digits.is_empty() {
                nums.push(digits.parse().expect("expected digits in column"));
            }
        }
        let op = find_op(op_row, s, e);
        total += match op {
            '*' => nums.iter().product::<u128>(),
            '+' => nums.iter().sum::<u128>(),
            _ => unreachable!(),
        };
    }
    total
}

#[cfg(test)]
mod tests {
    use super::{p1, p2};

    // Trailing spaces matter — preserve them.
    const EXAMPLE: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn p1_example() {
        assert_eq!(p1(EXAMPLE), 4_277_556);
    }

    #[test]
    fn p2_example() {
        // 1058 + 3253600 + 625 + 8544 = 3263827
        assert_eq!(p2(EXAMPLE), 3_263_827);
    }
}
