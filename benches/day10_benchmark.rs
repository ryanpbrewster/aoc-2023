use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn mybench(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/day10.input").unwrap();
    c.bench_function("day10/part2", |b| {
        b.iter(|| black_box(aoc_2023::day10::part2(&input)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = mybench,
}
criterion_main!(benches);
