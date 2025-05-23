use std::time::Instant;

mod core;
mod part_01;
mod part_02;

fn main() {
  let now = Instant::now();
  part_01::resolve("./input.data");
  let elapsed: std::time::Duration = now.elapsed();
  println!("Part 1 found in {:?}s", elapsed.as_secs());
  let now = Instant::now();
  part_02::resolve("FAILED :(");
  let elapsed = now.elapsed();
  println!("Part 2 found in {:?}s", elapsed.as_secs());
}
