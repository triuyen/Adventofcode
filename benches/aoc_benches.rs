use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_days::{aoc_day1, aoc_day2, aoc_day3, aoc_day4, aoc_day5, aoc_day7, aoc_day6, aoc_day8,aoc_day9, aoc_day10, aoc_day12};

fn bench_day1(c: &mut Criterion) {
    c.bench_function("day 1", |b| {
        b.iter(|| aoc_day1::mod_1::run())
    });

    c.bench_function("old day 1", |b| {
        b.iter(|| aoc_day1::mod_1_old1030::run())
    });

}

fn bench_day2(c: &mut Criterion) {
    c.bench_function("day 2", |b| {
        b.iter(|| aoc_day2::mod_2::run())
    });
}
fn bench_day3(c: &mut Criterion) {
    c.bench_function("day 3", |b| {
        b.iter(|| aoc_day3::mod_3::run())
    });
}

fn bench_day4(c: &mut Criterion) {
    c.bench_function("day 4", |b| {
        b.iter(|| aoc_day4::mod_4::run())
    });
}

fn bench_day5(c: &mut Criterion) {
    c.bench_function("day 5", |b| {
        b.iter(|| aoc_day5::mod_5::run())
    });
}

fn bench_day6(c: &mut Criterion) {
    c.bench_function("day 6", |b| {
        b.iter(|| aoc_day6::mod_6::run())
    });
}

fn bench_day8(c: &mut Criterion) {
    c.bench_function("day 8", |b| {
        b.iter(|| aoc_day8::mod_8::run())
    });
}

fn bench_day9(c: &mut Criterion) {
    c.bench_function("day 9", |b| {
        b.iter(|| aoc_day9::mod_9::run())
    });
}

fn bench_day10(c: &mut Criterion) {
    let input = std::fs::read_to_string("src/aoc_day10/input_day10.txt").unwrap();
    c.bench_function("day 10 p1", |b| {
        b.iter(|| aoc_day10::mod_10::p1(black_box(&input)))
    });
}

criterion_group!(benches, bench_day1,bench_day2,bench_day3,bench_day4,bench_day5, bench_day6, bench_day8,bench_day9, bench_day10);
criterion_main!(benches);
