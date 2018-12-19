use criterion::{criterion_group, criterion_main, Criterion};
use day_11::solve;

const INPUT: u32 = 5034;
const FUEL_CELL_GRID_SIZE: usize = 300;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve part two", |b| {
        b.iter(|| solve(FUEL_CELL_GRID_SIZE, INPUT, 1, 299))
    });
}

criterion_group! {name = benches; config = Criterion::default().sample_size(20); targets = criterion_benchmark}
criterion_main!(benches);
