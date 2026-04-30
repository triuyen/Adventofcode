
pub fn run() {
    let input = std::fs::read_to_string("src/aoc_day6/input_day6.txt")
        .expect("Cannot read input_day6.txt");
    println!("Day 6 p1 = {}", p1(&input));
    println!("Day 6 p2 = {}", p2(&input));
}

/// Parse the input into a flat byte grid padded with spaces.
/// Returns (buffer, width, height). Access cell (r, c) via buffer[r * width + c].
fn pad_grid_flat(input: &str) -> (Vec<u8>, usize, usize) {
    let bytes = input.as_bytes();

    // Pass 1: measure width (max line length) and height (non-empty line count).
    let mut width = 0;
    let mut height = 0;
    let mut line_start = 0;

    for i in 0..=bytes.len() {
        if i == bytes.len() || bytes[i] == b'\n' {
            let len = i - line_start;
            if len > 0 {
                if len > width { width = len; }
                height += 1;
            }
            line_start = i + 1;
        }
    }

    // One contiguous allocation, pre-filled with spaces (handles padding for free).
    let mut grid = vec![b' '; width * height];

    // Pass 2: copy each non-empty line into its row slot.
    let mut row = 0;
    let mut line_start = 0;
    for i in 0..=bytes.len() {
        if i == bytes.len() || bytes[i] == b'\n' {
            let len = i - line_start;
            if len > 0 {
                let dst = &mut grid[row * width..row * width + len];
                dst.copy_from_slice(&bytes[line_start..i]);
                row += 1;
            }
            line_start = i + 1;
        }
    }

    (grid, width, height)
}

/// Returns ranges [s, e) of columns belonging to a problem.
/// A column is a separator iff every row has a space at that column.
fn problem_ranges(grid: &[u8], width: usize, height: usize) -> Vec<(usize, usize)> {
    let is_sep = |col: usize| {
        (0..height).all(|r| grid[r * width + col] == b' ')
    };

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

/// Find the operator byte ('+' or '*') in the operator row's slice [s, e).
fn find_op(grid: &[u8], width: usize, op_row: usize, s: usize, e: usize) -> u8 {
    let row = &grid[op_row * width..(op_row + 1) * width];
    row[s..e]
        .iter()
        .copied()
        .find(|&b| b == b'+' || b == b'*')
        .expect("no operator found in problem range")
}

/// Parse a horizontal slice of a row as a u128, treating spaces as "no digits".
/// Returns None if the slice is all spaces.
fn parse_row_slice(grid: &[u8], width: usize, row: usize, s: usize, e: usize) -> Option<u128> {
    let slice = &grid[row * width + s..row * width + e];
    let mut value: u128 = 0;
    let mut has_digit = false;
    for &b in slice {
        if b == b' ' {
            continue;
        }
        debug_assert!(b.is_ascii_digit(), "unexpected non-digit byte: {}", b as char);
        value = value * 10 + (b - b'0') as u128;
        has_digit = true;
    }
    if has_digit { Some(value) } else { None }
}

/// Parse a vertical column [rows 0..height-1, fixed col] as a u128,
/// reading top-to-bottom (MSD -> LSD), skipping spaces.
fn parse_col(grid: &[u8], width: usize, height: usize, col: usize) -> Option<u128> {
    let mut value: u128 = 0;
    let mut has_digit = false;
    for r in 0..height {
        let b = grid[r * width + col];
        if b == b' ' {
            continue;
        }
        debug_assert!(b.is_ascii_digit(), "unexpected non-digit byte: {}", b as char);
        value = value * 10 + (b - b'0') as u128;
        has_digit = true;
    }
    if has_digit { Some(value) } else { None }
}

// ---------------- Part 1 ----------------
// Each data ROW within a problem is one number; numbers read horizontally.

pub fn p1(input: &str) -> u128 {
    let (grid, width, height) = pad_grid_flat(input);
    if height < 2 || width == 0 {
        return 0;
    }
    let op_row = height - 1;
    let ranges = problem_ranges(&grid, width, height);

    let mut total: u128 = 0;
    for (s, e) in ranges {
        let op = find_op(&grid, width, op_row, s, e);
        let mut acc_sum: u128 = 0;
        let mut acc_prod: u128 = 1;
        let mut count = 0;
        for r in 0..op_row {
            if let Some(n) = parse_row_slice(&grid, width, r, s, e) {
                acc_sum += n;
                acc_prod *= n;
                count += 1;
            }
        }
        if count == 0 { continue; }
        total += match op {
            b'*' => acc_prod,
            b'+' => acc_sum,
            _ => unreachable!(),
        };
    }
    total
}

// ---------------- Part 2 ----------------
// Each COLUMN within a problem is one number; digits top-to-bottom (MSD first),
// spaces skipped. + and * are commutative so column order is irrelevant.

pub fn p2(input: &str) -> u128 {
    let (grid, width, height) = pad_grid_flat(input);
    if height < 2 || width == 0 {
        return 0;
    }
    let op_row = height - 1;
    let ranges = problem_ranges(&grid, width, height);

    let mut total: u128 = 0;
    for (s, e) in ranges {
        let op = find_op(&grid, width, op_row, s, e);
        let mut acc_sum: u128 = 0;
        let mut acc_prod: u128 = 1;
        let mut count = 0;
        for col in s..e {
            // Only iterate the data rows (exclude the operator row).
            let mut value: u128 = 0;
            let mut has_digit = false;
            for r in 0..op_row {
                let b = grid[r * width + col];
                if b == b' ' { continue; }
                value = value * 10 + (b - b'0') as u128;
                has_digit = true;
            }
            if has_digit {
                acc_sum += value;
                acc_prod *= value;
                count += 1;
            }
        }
        if count == 0 { continue; }
        total += match op {
            b'*' => acc_prod,
            b'+' => acc_sum,
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
