mod aoc_day1 {
    pub mod mod_1;
}
mod aoc_day2 {
    pub mod mod_2;
    pub mod mod_2_0103;
}
mod aoc_day10 {
    pub mod mod_10;
}

mod aoc_day3 {
    pub mod mod_3;
}

mod aoc_day4 {
    pub mod mod_4;
}

mod aoc_day5 {
    pub mod mod_5;
}
mod aoc_day6{
    pub mod mod_6;
}

mod aoc_day8 {
    pub mod mod_8;
}

mod aoc_day7 {
    pub mod mod_7;
}



fn main() {
    //let input10 = std::fs::read_to_string("src/aoc_day10/input_day10.txt").unwrap();

    // Day 1 — has only run(), time it
    let start = std::time::Instant::now();
    aoc_day7::mod_7::run();
    println!("Day 7  [{:.2?}]", start.elapsed());

    // Day 2 — has only run(), time it  
    //let start = std::time::Instant::now();
    //aoc_day2::mod_2_0103::run();
    //println!("Day 2  [{:.2?}]", start.elapsed());

    //let start = std::time::Instant::now();
    //aoc_day3::mod_3::run();
    //println!("Day 3  [{:.2?}]", start.elapsed());
    
    //let start = std::time::Instant::now();
    //aoc_day5::mod_5::run();
    //println!("Day 5  [{:.2?}]", start.elapsed());

    //let start = std::time::Instant::now();
    //aoc_day4::mod_4::run();
    //println!("Day 4 [{:.2?}]", start.elapsed());

    // Day 10 — has p1/p2
    //let start = std::time::Instant::now();
    //println!("Day 10 p1 = {}  [{:.2?}]", aoc_day10::mod_10::p1(&input10), start.elapsed());
    //

}
