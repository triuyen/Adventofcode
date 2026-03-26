use std::io::{self, BufRead};

pub fn run() {
    let stdin = io::stdin();
    let mut pos: i64 = 50;
    let mut count = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
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

    println!("{count}");
}
