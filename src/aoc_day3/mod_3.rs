pub fn run() {
    let input = std::fs::read_to_string("src/aoc_day3/input_day3.txt")
        .expect("Cannot read input_day3.txt");
    println!("Day 3 p1 = {}", p1(&input));
    println!("Day 3 p2 = {}", p2(&input));
}


fn max_subsequence(line: &str, keep: usize) -> String {
    let digits: Vec<char> = line.trim().chars().collect();

    if digits.len() <= keep {
        return digits.into_iter().collect();
    }

    let mut to_remove = digits.len() - keep;
    let mut stack: Vec<char> = Vec::with_capacity(digits.len());

    for &c in &digits {
        while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < c {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(c);
    }

    stack.truncate(keep);
    stack.into_iter().collect()
}

pub fn p1(input: &str) -> u128 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let best = max_subsequence(line, 2);  // ← keep only 2 digits
            best.parse::<u128>().unwrap()
        })
        .sum()
}

pub fn p2(input: &str) -> u128 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let best = max_subsequence(line, 12); // ← only change from p1
            best.parse::<u128>().unwrap()
        })
        .sum()
}
