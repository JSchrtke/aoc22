use aoc22::day1::find_biggest_sum;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_finding_biggest_sum(c: &mut Criterion) {
    let input =
        String::from("1000/n2000/n3000/n/n4000/n/n5000/n6000/n/n7000/n8000/n9000/n/n10000/n");
    c.bench_function("finding_biggest_sum", |b| {
        b.iter(|| find_biggest_sum(black_box(&input)))
    });
}

criterion_group!(benches, bench_finding_biggest_sum);
criterion_main!(benches);
