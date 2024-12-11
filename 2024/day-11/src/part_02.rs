use std::collections::HashMap;

use crate::core::parse_number_list;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let mut stones = transform_data(data);

  for _ in 1..=75 {
    stones = blink(stones);
  }

  let final_result = stones.values().fold(0, |sum, stone_number| sum + (1 * stone_number));

  println!("Part 2 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> HashMap<u64, u64> {
    let v: Vec<u64> = parse_number_list(&data[0]);

    let mut hash = HashMap::new();
    for x in v {
      hash.entry(x).and_modify(|e| *e += 1).or_insert(1);
    }
    hash
}

fn get_number_length(number: u64) -> u32 {
  let mut len = 1;
  let mut x = number;

  while x / 10 > 0 {
    len += 1;
    x = x / 10;
  }

  len
}

fn split_number(number: u64) -> Vec<u64> {
  let len = get_number_length(number);

  let split_length = len / 2;
  let div = 10_u64.pow(split_length);

  vec![number / div, number % div]
}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
  let mut result = HashMap::new();

  for k in stones.keys() {
    let s = stones.get(&k).unwrap();

    let new_stones = transform_stone(*k);
    for engraved in new_stones {
      result.entry(engraved).and_modify(|e| *e += 1 * s).or_insert(1 * s);
    }

  }

  result
}
fn transform_stone(stone: u64) -> Vec<u64> {
  if stone == 0 { vec![1] }
  else if get_number_length(stone) % 2 == 0 { split_number(stone) }
  else { vec![stone * 2024] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_length_01() {
      assert_eq!(1, get_number_length(0));
    }
    #[test]
    fn number_length_02() {
      assert_eq!(2, get_number_length(10));
    }
    #[test]
    fn number_length_03() {
      assert_eq!(2, get_number_length(12));
    }
    #[test]
    fn number_length_04() {
      assert_eq!(6, get_number_length(235000));
    }


    #[test]
    fn split_number_01() {
      assert_eq!(vec![1,0], split_number(10));
    }
    #[test]
    fn split_number_02() {
      assert_eq!(vec![1,2], split_number(12));
    }
    #[test]
    fn split_number_03() {
      assert_eq!(vec![253,0], split_number(253000));
    }
    #[test]
    fn split_number_example_01() {
      assert_eq!(vec![1,7], split_number(17));
    }
    #[test]
    fn split_number_example_02() {
      assert_eq!(vec![512,72], split_number(512072));
    }
    #[test]
    fn split_number_example_03() {
      assert_eq!(vec![80,96], split_number(8096));
    }
}
