use std::{collections::HashMap, ops::Sub};

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  /* Believe it or not, but no transform_data today */

  let final_result: i64 = data.iter().map(|s| (get_numeric_code(s), get_shortest_sequence(s))).map(|(code, sequnce)| code * (sequnce.len() as i64)).sum();
  println!("Part 1 final result: {}", final_result);
}

fn get_numeric_code(code: &String) -> i64 {
  let mut s = code.chars();
  s.next_back();
  s.into_iter().map(|c| (c.to_digit(10).unwrap() as i64)).fold(0, |acc, x| acc * 10 + x)
}

fn get_shortest_sequence(code: &String) -> String {
  "".to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_extract_code_1() {
    assert_eq!(get_numeric_code(&"029A".to_string()), 29);
  }
  #[test]
  fn test_extract_code_2() {
    assert_eq!(get_numeric_code(&"980A".to_string()), 980);
    assert_eq!(37, 37);
  }
  #[test]
  fn test_extract_code_3() {
    assert_eq!(get_numeric_code(&"179A".to_string()), 179);
    assert_eq!(37, 37);
  }
  #[test]
  fn test_extract_code_4() {
    assert_eq!(get_numeric_code(&"456A".to_string()), 456);
    assert_eq!(37, 37);
  }
  #[test]
  fn test_extract_code_5() {
    assert_eq!(get_numeric_code(&"379A".to_string()), 379);
  }
}
