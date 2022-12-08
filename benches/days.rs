use advent_of_code_2022::days::{
    day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8, read_day_input,
};
use advent_of_code_2022::run_all_days;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_day_1(c: &mut Criterion) {
    let input = read_day_input("day_1");
    c.bench_function("day 1", |b| b.iter(|| day_1::solve(black_box(&input))));
}

fn bench_day_2(c: &mut Criterion) {
    let input = read_day_input("day_2");
    c.bench_function("day 2", |b| b.iter(|| day_2::solve(black_box(&input))));
}

fn bench_day_3(c: &mut Criterion) {
    let input = read_day_input("day_3");
    c.bench_function("day 3", |b| b.iter(|| day_3::solve(black_box(&input))));
}

fn bench_day_4(c: &mut Criterion) {
    let input = read_day_input("day_4");
    c.bench_function("day 4", |b| b.iter(|| day_4::solve(black_box(&input))));
}

fn bench_day_5(c: &mut Criterion) {
    let input = read_day_input("day_5");
    c.bench_function("day 5", |b| b.iter(|| day_5::solve(black_box(&input))));
}

fn bench_day_6(c: &mut Criterion) {
    let input = read_day_input("day_6");
    c.bench_function("day 6", |b| b.iter(|| day_6::solve(black_box(&input))));
}

fn bench_day_7(c: &mut Criterion) {
    let input = read_day_input("day_7");
    c.bench_function("day 7", |b| b.iter(|| day_7::solve(black_box(&input))));
}

fn bench_day_8(c: &mut Criterion) {
    let input = read_day_input("day_8");
    c.bench_function("day 8", |b| b.iter(|| day_8::solve(black_box(&input))));
}

fn bench_all_days(c: &mut Criterion) {
    c.bench_function("All days", |b| b.iter(|| run_all_days()));
}

criterion_group!(
    benches,
    bench_all_days,
    bench_day_1,
    bench_day_2,
    bench_day_3,
    bench_day_4,
    bench_day_5,
    bench_day_6,
    bench_day_7,
    bench_day_8,
);
criterion_main!(benches);
