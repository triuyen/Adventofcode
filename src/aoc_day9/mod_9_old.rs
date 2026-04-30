// src/aoc_day9/mod_9.rs

pub fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (x, y) = l.split_once(',').expect("expected `x,y`");
            (
                x.trim().parse().expect("invalid x"),
                y.trim().parse().expect("invalid y"),
            )
        })
        .collect()
}

pub fn p1(points: &[(i64, i64)]) -> i64 {
    let mut best = 0i64;
    for i in 0..points.len() {
        let (x1, y1) = points[i];
        for j in (i + 1)..points.len() {
            let (x2, y2) = points[j];
            let w = (x1 - x2).abs() + 1;
            let h = (y1 - y2).abs() + 1;
            let area = w * h;
            if area > best {
                best = area;
            }
        }
    }
    best
}


pub fn run() {
    let raw = std::fs::read_to_string("src/aoc_day9/input_day9.txt")
        .expect("could not read input_day9.txt");
    let pts = parse(&raw);
    println!("Day 9 p1 = {}", p1(&pts));
}


