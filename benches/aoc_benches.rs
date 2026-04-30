use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_days::{aoc_day5, aoc_day6, aoc_day7, aoc_day9, aoc_day10};


fn bench_day5(c: &mut Criterion){
    c.bench_function("day 5", |b|{
        b.iter(|| aoc_day5::mod_5::run())
    });
}

fn bench_day6(c: &mut Criterion){
    c.bench_function("day 6", |b|{
        b.iter(|| aoc_day6::mod_6::run())
    });
}

fn bench_day7(c: &mut Criterion){
    c.bench_function("day 7", |b|{
        b.iter(|| aoc_day7::mod_7::run())
    });
}


fn bench_day9(c: &mut Criterion){
    c.bench_function("day 9", |b|{
        b.iter(|| aoc_day9::mod_9::run())
    });
}

fn bench_day10(c: &mut Criterion ){
    let input = std::fs::read_to_string("src/aoc_day/input_day10.txt").unwrap();
    c.bench_function("day 10 p1", |b| {
        b.iter(||aoc_day10::mod_10::p1(black_box(&input)))
    });
}

criterion_group!(benches, bench_day7);
criterion_main!(benches);
