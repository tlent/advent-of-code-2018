#[macro_use]
extern crate criterion;
extern crate day_1;

use day_1::{find_repeat_result, parse_input};

use criterion::Criterion;

pub const INPUT: &'static str = include_str!("../input");

fn criterion_benchmark(c: &mut Criterion) {
  let numbers = parse_input(INPUT).unwrap();
  c.bench_function("find_repeat_result", move |b| {
    b.iter(|| find_repeat_result(&numbers))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
