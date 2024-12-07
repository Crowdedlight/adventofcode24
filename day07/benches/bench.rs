use criterion::{black_box, Criterion, criterion_group, criterion_main};
use day07::*;

fn part1_benchmark(c: &mut Criterion) {
    let input1 = black_box(include_str!("../input1.txt"));

    c.bench_function("part 1", |b| b.iter(|| part1::process(input1)));
}

fn part2_benchmark(c: &mut Criterion) {
    let input2 = black_box(include_str!("../input2.txt"));
    c.bench_function("part 2", |b| b.iter(|| part2::process(input2)));
}

criterion_group!(benches, part1_benchmark, part2_benchmark);
criterion_main!(benches);