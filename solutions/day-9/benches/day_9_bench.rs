use criterion::{criterion_group, criterion_main, Criterion, Fun};
use day_9::{solve_with_deque, solve_with_linked_list};

const PART_ONE_INPUT: (usize, u32) = (412, 71646);
const PART_TWO_INPUT: (usize, u32) = (PART_ONE_INPUT.0, PART_ONE_INPUT.1 * 100);

fn criterion_benchmark(c: &mut Criterion) {
  let with_deque = Fun::new("with deque", |b, i| b.iter(|| solve_with_deque(i)));
  let with_linked_list = Fun::new("with linked list", |b, i| {
    b.iter(|| solve_with_linked_list(i))
  });
  c.bench_functions("solve", vec![with_deque, with_linked_list], PART_TWO_INPUT);
}

criterion_group! {name = benches; config = Criterion::default().sample_size(20); targets = criterion_benchmark}
criterion_main!(benches);
