fn split_into_problems(input: &str) -> Vec<Vec<String>> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    if lines.is_empty() {
        return Vec::new();
    }

    let height = lines.len();
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut padded = vec![vec![' '; width]; height];
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.iter().enumerate() {
            padded[r][c] = *ch;
        }
    }

    let mut problems = Vec::new();
    let mut start = None;

    for c in 0..width {
        let is_blank_column = (0..height).all(|r| padded[r][c] == ' ');

        match (start, is_blank_column) {
            (None, false) => start = Some(c),
            (Some(s), true) => {
                let block = extract_block(&padded, s, c);
                if !block.is_empty() {
                    problems.push(block);
                }
                start = None;
            }
            _ => {}
        }
    }

    if let Some(s) = start {
        let block = extract_block(&padded, s, width);
        if !block.is_empty() {
            problems.push(block);
        }
    }

    problems
}

fn extract_block(grid: &[Vec<char>], start: usize, end: usize) -> Vec<String> {
    grid.iter()
        .map(|row| row[start..end].iter().collect::<String>())
        .collect()
}

fn solve_problem_p1(block: &[String]) -> u128 {
    if block.is_empty() {
        return 0;
    }

    let op_line = block.last().unwrap().trim();
    let op = op_line
        .chars()
        .find(|&c| c == '+' || c == '*')
        .unwrap_or_else(|| panic!("Opérateur introuvable dans {:?}", op_line));

    let numbers: Vec<u128> = block[..block.len() - 1]
        .iter()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(
                    trimmed
                        .parse::<u128>()
                        .unwrap_or_else(|e| panic!("Nombre invalide {:?}: {}", trimmed, e)),
                )
            }
        })
        .collect();

    match op {
        '+' => numbers.into_iter().sum(),
        '*' => numbers.into_iter().product(),
        _ => unreachable!(),
    }
}

fn solve_problem_p2(block: &[String]) -> u128 {
    if block.is_empty() {
        return 0;
    }

    let rows = block.len();
    let cols = block.iter().map(|line| line.len()).max().unwrap_or(0);

    let grid: Vec<Vec<char>> = block
        .iter()
        .map(|line| {
            let mut row: Vec<char> = line.chars().collect();
            row.resize(cols, ' ');
            row
        })
        .collect();

    let op = grid[rows - 1]
        .iter()
        .copied()
        .find(|&c| c == '+' || c == '*')
        .unwrap_or_else(|| panic!("Opérateur introuvable dans la dernière ligne"));

    let mut numbers = Vec::new();

    for c in 0..cols {
        let mut s = String::new();

        for row in grid.iter().take(rows - 1) {
            let ch = row[c];
            if ch.is_ascii_digit() {
                s.push(ch);
            }
        }

        if !s.is_empty() {
            numbers.push(
                s.parse::<u128>()
                    .unwrap_or_else(|e| panic!("Nombre invalide {:?}: {}", s, e)),
            );
        }
    }

    match op {
        '+' => numbers.into_iter().sum(),
        '*' => numbers.into_iter().product(),
        _ => unreachable!(),
    }
}

pub fn p1(input: &str) -> u128 {
    split_into_problems(input)
        .iter()
        .map(|block| solve_problem_p1(block))
        .sum()
}

pub fn p2(input: &str) -> u128 {
    split_into_problems(input)
        .iter()
        .map(|block| solve_problem_p2(block))
        .sum()
}

#[cfg(test)]
mod test {
    use super::{p1, p2};

    #[test]
    fn p1_test() {
        let input = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        assert_eq!(p1(input), 4277556);
    }

    #[test]
    fn p2_test() {
        let input = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        assert_eq!(p2(input), 3263827);
    }
}