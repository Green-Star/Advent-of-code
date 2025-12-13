use std::time::Instant;
use day_12::part_01;

fn main() {
  let now = Instant::now();
  let result = part_01::resolve(include_str!("../input.data"));
  let elapsed: std::time::Duration = now.elapsed();
  println!("Part 1 final result: {result}");
  println!("Part 1 found in {:?}s", elapsed.as_secs());
}
