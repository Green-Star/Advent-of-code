use std::time::Instant;
use day_02::part_01;
use day_02::part_02;

fn main() {
  let now = Instant::now();
  let result = part_01::resolve(include_str!("../input.data"));
  println!("Part 1 final result: {result}");
  let elapsed: std::time::Duration = now.elapsed();
  println!("Part 1 found in {:?}s", elapsed.as_secs());

  let now = Instant::now();
  let result = part_02::resolve(include_str!("../input.data"));
  println!("Part 2 final result: {result}");
  let elapsed = now.elapsed();
  println!("Part 2 found in {:?}s", elapsed.as_secs());
}
