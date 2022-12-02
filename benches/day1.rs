use aoc22::day1::solve_part_1;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_finding_biggest_sum(c: &mut Criterion) {
    let input = std::fs::read_to_string("input.txt").unwrap();
    c.bench_function("finding_biggest_sum", |b| {
        b.iter(|| solve_part_1(black_box(&input)))
    });
}

criterion_group!(benches, bench_finding_biggest_sum);
criterion_main!(benches);
