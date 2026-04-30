

pub fn run() {
    // le run principale qui import le fichier input et les fonction en dessous
    let input = std::fs::read_to_string("src/aoc_day1/input_day1.txt")
        .expect("Cannot read input_day1.txt");
    println!("Running Day 1");
    println!("run P1:{}", p1(&input));

}

// naive method . Using the same logic as in p1 but counting how many times we pass through 0 during the rotation
pub fn p1(input: &str) -> i64 {
    let mut pos: i64 = 50;
    let mut count: i64 = 0;

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