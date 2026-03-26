use std::io::{self, Read};

pub fn run() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

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

    // A "double" number of half-length n = base * (10^n + 1)
    // e.g. n=1: 11,22,...,99  n=2: 1010,1111,...,9999
    for n in 1u32..=10 {
        let pow_n = 10u128.pow(n);
        let multiplier = pow_n + 1;
        let base_min = if n == 1 { 1u128 } else { pow_n / 10 };
        let base_max = pow_n - 1;

        for &(a, b) in &ranges {
            let lo = ((a + multiplier - 1) / multiplier).max(base_min);
            let hi = (b / multiplier).min(base_max);
            if lo > hi { continue; }

            // Sum of (base * multiplier) for base in lo..=hi
            let count = hi - lo + 1;
            let sum = count * (lo + hi) / 2;
            total += multiplier * sum;
        }
    }

    println!("{total}");
}
