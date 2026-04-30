pub fn run() {
    let input = std::fs::read_to_string("src/aoc_day5/input_day5.txt")
        .expect("Cannot read input_day5.txt");
    println!("Day 5 p2 = {}", p2(&input));
}

pub fn p1(input: &str) -> usize {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    let mut reading_ranges = true;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { reading_ranges = false; continue; }
        if reading_ranges {
            let (a, b) = line.split_once('-').unwrap();
            let a = a.trim().parse::<u64>().unwrap();
            let b = b.trim().parse::<u64>().unwrap();
            ranges.push((a, b));
        } else {
            ids.push(line.parse::<u64>().unwrap());
        }
    }

    // Step 1 — sort ranges by start
    ranges.sort_unstable_by_key(|&(a, _)| a);

    // Step 2 — merge overlapping ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (a, b) in ranges {
        if let Some(last) = merged.last_mut() {
            if a <= last.1 + 1 {
                last.1 = last.1.max(b); // extend
                continue;
            }
        }
        merged.push((a, b));
    }

    // Step 3 — binary search per ID
    ids.iter()
        .filter(|&&id| {
            // Find the last range whose start <= id
            let pos = merged.partition_point(|&(a, _)| a <= id);
            if pos == 0 { return false; }
            let (_, b) = merged[pos - 1];
            id <= b
        })
        .count()
}


pub fn p2(input: &str) -> u64 {
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    // Only read ranges, stop at blank line
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { break; }
        let (a, b) = line.split_once('-').unwrap();
        let a = a.trim().parse::<u64>().unwrap();
        let b = b.trim().parse::<u64>().unwrap();
        ranges.push((a, b));
    }

    // Sort by start
    ranges.sort_unstable_by_key(|&(a, _)| a);

    // Merge overlapping ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (a, b) in ranges {
        if let Some(last) = merged.last_mut() {
            if a <= last.1 + 1 {
                last.1 = last.1.max(b);
                continue;
            }
        }
        merged.push((a, b));
    }

    // Sum lengths of all merged ranges
    merged.iter().map(|(a, b)| b - a + 1).sum()
}
