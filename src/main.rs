//mod day1;
//mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

fn main()
{
    //let input = std::fs::read_to_string("src/day1/d1lesly.txt")
    //    .expect("Impossible de lire src/day1/d1lesly.txt");

    //println!("Stops sur 0 (p1)  : {}", day1::p1(&input));
    //println!("Passages par 0 (p2): {}", day1::p2(&input));


    // let input_day2 = std::fs::read_to_string("src/day2/day2.txt")
    //     .expect("Impossible de lire src/day2/day2.txt");
    //
    // println!("Day 2 - p1 = {}", day2::p1(&input_day2));
    // println!("Day 2 - p2 = {}", day2::p2(&input_day2));


    // let input_day3 = std::fs::read_to_string("src/day3/day3.txt")
    //     .expect("Impossible de lire src/day3/day3.txt");
    //
    // println!("Day 3 - p1 = {}", day3::p1(&input_day3));
    // println!("Day 3 - p2 = {}", day3::p2(&input_day3));

    // let input_day4 = std::fs::read_to_string("src/day4/day4.txt")
    //     .expect("Impossible de lire src/day4/day4.txt");
    //
    // println!("Day 4 - p1 = {}", day4::p1(&input_day4));
    // println!("Day 4 - p2 = {}", day4::p2(&input_day4));

    // let input_day5 = std::fs::read_to_string("src/day5/day5.txt")
    //     .expect("Impossible de lire src/day5/day5.txt");
    //
    // println!("Day 5 - p1 = {}", day5::p1(&input_day5));
    // println!("Day 5 - p2 = {}", day5::p2(&input_day5));

    // let input_day6 = std::fs::read_to_string("src/day6/day6.txt")
    //     .expect("Impossible de lire src/day6/day6.txt");
    //
    // println!("Day 6 - p1 = {}", day6::p1(&input_day6));
    // println!("Day 6 - p2 = {}", day6::p2(&input_day6));

    // let input_day7 = std::fs::read_to_string("src/day7/day7.txt")
    //     .expect("Impossible de lire src/day7/day7.txt");
    //
    // println!("Day 7 - p1 = {}", day7::p1(&input_day7));
    // println!("Day 7 - p2 = {}", day7::p2(&input_day7));

    // let input_day8 = std::fs::read_to_string("src/day8/day8.txt")
    //     .expect("Impossible de lire src/day8/day8.txt");
    //
    // println!("Day 8 - p1 = {}", day8::p1(&input_day8));
    // println!("Day 8 - p2 = {}", day8::p2(&input_day8));
    //
    // let input_day9 = std::fs::read_to_string("src/day9/day9.txt")
    //     .expect("Impossible de lire src/day9/day9.txt");
    //
    // println!("Day 9 - p1 = {}", day9::p1(&input_day9));
    // println!("Day 9 - p2 = {}", day9::p2(&input_day9));
    //
    // let input_day10 = std::fs::read_to_string("src/day10/day10.txt")
    //     .expect("Impossible de lire src/day10/day10.txt");
    //
    // println!("avant p1");
    // let r1 = day10::p1(&input_day10);
    // println!("Day 10 - p1 = {}", r1);
    //
    // println!("avant p2");
    // let r2 = day10::p2(&input_day10);
    // println!("Day 10 - p2 = {}", r2);

    //
    // let input_day11 = std::fs::read_to_string("src/day11/day11.txt")
    //     .expect("Impossible de lire src/day11/day11.txt");
    //
    // println!("Day 11 - p1 = {}", day11::p1(&input_day11));
    // println!("Day 11 - p2 = {}", day11::p2(&input_day11));
    //
    let input_day12 = std::fs::read_to_string("src/day12/day12.txt")
        .expect("Impossible de lire src/day12/day12.txt");
    
    println!("Day 12 - p1 = {}", day12::p1(&input_day12));





    //println!("day1p1");
}

// fn count_zero_events(instructions: &[&str]) -> (u32, u32) {
//     let mut position: i32 = 0;
//     let mut passes: u32 = 0;
//     let mut stops: u32 = 0;
//
//     for instr in instructions {
//         let (dir, value) = instr.split_at(1);
//         let steps: i32 = value.parse().unwrap();
//
//         match dir {
//             "R" => {
//                 // passage par 0
//                 if position + steps >= 100 {
//                     passes += 1;
//                 }
//                 position = (position + steps) % 100;
//             }
//             "L" => {
//                 // passage par 0
//                 if steps > position {
//                     passes += 1;
//                 }
//                 position = (position - steps).rem_euclid(100);
//             }
//             _ => {}
//         }
//
//         // arrêt exact sur 0
//         if position == 0 {
//             stops += 1;
//         }
//     }
//
//     (passes, stops)
// }
