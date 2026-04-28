pub fn run() {
    let input = std::fs::read_to_string("src/aoc_day6/input_day6.txt")
        .expect("Cannot read input_day6.txt");
    println!("Day 6 p1 = {}", p1(&input));
}

/// Parse the worksheet as a uniform grid of chars.
/// Each line is right-padded with spaces to the maximum line width so
/// column-indexing across rows is always safe.
fn pad_grid(input: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty()) // drop only fully empty lines, keep all-space ones
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

/// Find maximal runs of columns that are NOT separators.
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

/// Trim a slice of chars and turn it into a String.
fn slice_trim(row: &[char], s: usize, e: usize) -> String {
    row[s..e].iter().collect::<String>().trim().to_string()
}

pub fn p1(input: &str) -> u128 {
    let grid = pad_grid(input);
    if grid.len() < 2 {
        return 0;
    }

    let (op_row, data_rows) = grid.split_last().unwrap();
    let ranges = problem_ranges(&grid);

    let mut total: u128 = 0;
    for (s, e) in ranges {
        // Numbers: one per data row, skipping rows that are blank in this column range
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

        let op = slice_trim(op_row, s, e);
        total += match op.as_str() {
            "*" => nums.iter().product::<u128>(),
            "+" => nums.iter().sum::<u128>(),
            other => panic!("Unknown operator {:?} in columns {}..{}", other, s, e),
        };
    }
    total
}

#[cfg(test)]
mod tests {
    use super::p1;

    #[test]
    fn p1_example() {
        // Trailing spaces matter — keep them.
        let input = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
        assert_eq!(p1(input), 4_277_556);
    }
}
