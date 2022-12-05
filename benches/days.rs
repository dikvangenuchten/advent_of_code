use advent_of_code_2022::days::{day_1, day_2, day_3, day_4, day_5, read_day_input};

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

criterion_group!(
    benches,
    bench_day_1,
    bench_day_2,
    bench_day_3,
    bench_day_4,
    bench_day_5
);
criterion_main!(benches);
