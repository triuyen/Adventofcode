pub fn run() {
    // le run principale qui import le fichier input et les fonction en dessous
    let input = std::fs::read_to_string("src/aoc_day1/input_day1.txt")
        .expect("Cannot read input_day1.txt");
    println!("Running Day 1");
    println!("run P2:{}", p2(&input));
}

pub fn p1(input: &str) -> i64 {
    let mut pos: i64 = 50;
    let mut count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (dir, dist) = line.split_at(1);
        let dist: i64 = dist.parse().expect("invalid number");

        pos = match dir {
            "L" => (pos - dist).rem_euclid(100),
            "R" => (pos + dist).rem_euclid(100),
            _ => panic!("unknown direction: {}", dir),
        };

        if pos == 0 {
            count += 1;
        }
    }

    count
}

// ---- part 2 of day 1

pub fn p2(input: &str) -> i64 {
    let mut pos: i64 = 50;
    let mut count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        let (dir, dist) = line.split_at(1);
        let dist: i64 = dist.parse().expect("invalid number");

        // Count how many times we pass through 0 during this rotation
        count += count_hits_zero(pos, dir, dist);

        // Update position
        pos = match dir {
            "L" => (pos - dist).rem_euclid(100),
            "R" => (pos + dist).rem_euclid(100),
            _ => panic!("unknown direction: {}", dir),
        };
    }

    count
}

fn count_hits_zero(pos: i64, dir: &str, steps: i64) -> i64 {
    if steps <= 0 { return 0; }

    // How many steps until we first hit 0?
    let first_hit = match dir {
        "L" => if pos == 0 { 100 } else { pos },
        "R" => if pos == 0 { 100 } else { 100 - pos },
        _ => panic!("unknown direction"),
    };

    if steps < first_hit {
        // Never reaches 0
        0
    } else {
        // First hit + every 100 steps after
        1 + (steps - first_hit) / 100
    }
}
