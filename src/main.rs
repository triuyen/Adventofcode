
use aoc_days::{aoc_day6, aoc_day10};

fn main(){
    let start = std::time::Instant::now();
    aoc_day6::mod_6::run();
    println!("Day 6 [{:.2?}]", start.elapsed());
}
