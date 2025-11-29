use std::time::Instant;
use day_01::part_01;
use day_01::part_02;

fn main() {
  let now = Instant::now();
//  part_01::resolve("./input.data");
  let elapsed: std::time::Duration = now.elapsed();
  println!("Part 1 found in {:?}s", elapsed.as_secs());
  let now = Instant::now();
//  part_02::resolve("./input.data");
  let elapsed = now.elapsed();
  println!("Part 2 found in {:?}s", elapsed.as_secs());
}
