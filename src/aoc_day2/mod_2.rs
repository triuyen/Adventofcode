//use std::io::{self, Read};


pub fn run() {
    let input = std::fs::read_to_string("src/aoc_day2/input_day2.txt")
        .expect("Cannot read input_day2.txt");
    
    println!("Day 2 p1 = {}", p1(&input));
    println!("Day 2 p2 = {}", p2(&input));
}


pub fn is_invalid_id_p1(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    s[..half] == s[half..]
}


pub fn p1(input: &str) -> u64 {
    let line = input.trim();
    if line.is_empty() {
        return 0;
    }

    let mut sum = 0u64;

    for range in line.split(',') {
        let range = range.trim();
        if range.is_empty() {
            continue;
        }

        let (start, end) = range
            .split_once('-')
            .unwrap_or_else(|| panic!("Range invalide: {:?}", range));

        let start: u64 = start
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("Début invalide dans {:?}: {}", range, e));

        let end: u64 = end
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("Fin invalide dans {:?}: {}", range, e));

        for n in start..=end {
            if is_invalid_id_p1(n) {
                sum += n;
            }
        }
    }

    sum
}

// only day 2 part 2 is implemented here, part 1 is on going
pub fn p2(input: &str) -> u128 {
    let ranges: Vec<(u128, u128)> = input
        .split(',')
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() { return None; }
            let mut parts = s.splitn(2, '-');
            let a = parts.next()?.trim().parse::<u128>().ok()?;
            let b = parts.next()?.trim().parse::<u128>().ok()?;
            Some((a, b))
        })
        .collect();

    let mut total: u128 = 0;

    for n in 1u32..=10 {
        let pow_n = 10u128.pow(n);
        let multiplier = pow_n + 1;
        let base_min = if n == 1 { 1u128 } else { pow_n / 10 };
        let base_max = pow_n - 1;

        for &(a, b) in &ranges {
            let lo = ((a + multiplier - 1) / multiplier).max(base_min);
            let hi = (b / multiplier).min(base_max);
            if lo > hi { continue; }

            let count = hi - lo + 1;
            let sum = count * (lo + hi) / 2;
            total += multiplier * sum;
        }
    }

    total
}
