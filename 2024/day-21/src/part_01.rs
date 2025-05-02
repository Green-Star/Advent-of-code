use std::collections::HashMap;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  /* Believe it or not, but no transform_data today */

  let final_result: i64 = data.iter().map(|s| (get_numeric_code(s), get_shortest_sequence(s, 2))).map(|(code, sequnce)| code * (sequnce.len() as i64)).sum();
  println!("Part 1 final result: {}", final_result);
}

fn get_numeric_code(code: &String) -> i64 {
  let mut s = code.chars();
  s.next_back();
  s.into_iter().map(|c| (c.to_digit(10).unwrap() as i64)).fold(0, |acc, x| acc * 10 + x)
}

fn get_shortest_sequence(code: &String, robots: u64) -> String {
  let mut sequence = get_sequence_for_numeric_keypad(code);

  for _ in 1..=robots {
    sequence = get_sequence_for_directionnal_keypad(&sequence);
  }

  sequence
}
fn get_sequence_for_numeric_keypad(code: &String) -> String {
  let keypad = NumericKeypad::create_numeric_keypad();

  /* Always start on A */
  let mut current = keypad.get(&'A').unwrap();

  let mut sequence = String::new();
  for c in code.chars() {
    let destination = keypad.get(&c).unwrap();

    let offset_x = destination.x.abs_diff(current.x);
    let offset_y = destination.y.abs_diff(current.y);

    let mut horizontal = String::new();
    for _ in 0..offset_x {
      if destination.x >= current.x {
        horizontal.push('>');
      } else {
        horizontal.push('<');
      }
    }
    let mut vertical = String::new();
    for _ in 0..offset_y {
      if destination.y >= current.y {
        vertical.push('^');
      } else {
        vertical.push('v');
      }
    }

    /* Direction priorities:
      1. Moving with least turns
      2. Then, inside a move:
        < before
        ^ before
        v before
        >
     */
    if current.y == 0 && destination.x == 0 {
      sequence.push_str(&vertical);
      sequence.push_str(&horizontal);
    } else if current.x == 0 && destination.y == 0 {
      sequence.push_str(&horizontal);
      sequence.push_str(&vertical);
    } else if current.x < destination.x {
      sequence.push_str(&horizontal);
      sequence.push_str(&vertical);
    } else if current.x >= destination.x {
      sequence.push_str(&vertical);
      sequence.push_str(&horizontal);
    }

    sequence.push('A');
    current = destination;
  }
  sequence
}
fn get_sequence_for_directionnal_keypad(sequence: &String) -> String {
  let steps = sequence.split_inclusive("A").map(|s| s.to_string()).collect::<Vec<_>>();
  let mut hashmap = HashMap::new();
  steps.iter().map(|s| get_next_sequence(s, &mut hashmap)).collect::<Vec<_>>().join("")
}
fn get_next_sequence(key: &String, hashmap: &mut HashMap<String, String>) -> String {
  if let Some(next) = hashmap.get(key) {
    next.clone()
  } else {
    let next = compute_next_value(key);
    hashmap.insert(key.clone(), next.clone());
    next
  }
}
fn compute_next_value(s: &String) -> String {
  "v<<A".to_string()
}


#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
  x: usize,
  y: usize,
}

/*
      +---+---+---+
  3:  | 7 | 8 | 9 |
      +---+---+---+
  2:  | 4 | 5 | 6 |
      +---+---+---+
  1:  | 1 | 2 | 3 |
      +---+---+---+
  0: 	    | 0 | A |
Y^	      +---+---+
  X>    0   1   2
*/
type NumericKeypad = HashMap<char, Position>;
trait NumericKeypadTrait {
  fn create_numeric_keypad() -> Self;
}
impl NumericKeypadTrait for NumericKeypad {
  fn create_numeric_keypad() -> Self {
    HashMap::from([
      ('7', Position { x: 0, y: 3 }), ('8', Position { x: 1, y: 3 }), ('9', Position { x: 2, y: 3 }),
      ('4', Position { x: 0, y: 2 }), ('5', Position { x: 1, y: 2 }), ('6', Position { x: 2, y: 2 }),
      ('1', Position { x: 0, y: 1 }), ('2', Position { x: 1, y: 1 }), ('3', Position { x: 2, y: 1 }),
                                      ('0', Position { x: 1, y: 0 }), ('A', Position { x: 2, y: 0 }),
    ])
  }
}

/*
          +---+---+
  1:      | ^ | A |
      +---+---+---+
  0:  | < | v | > |
Y^    +---+---+---+
  X>    0   1   2
*/
type DirectionalKeypad = HashMap<char, Position>;
trait DirectionalKeypadTrait {
  fn create_directional_keypad() -> Self;
}
impl DirectionalKeypadTrait for DirectionalKeypad {
  fn create_directional_keypad() -> Self {
    HashMap::from([
                                      ('^', Position { x: 1, y: 1 }), ('A', Position { x: 2, y: 1 }),
      ('<', Position { x: 0, y: 0 }), ('v', Position { x: 1, y: 0 }), ('>', Position { x: 2, y: 0 }),
    ])
  }
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
  #[test]
  fn shortest_sequence_1() {
    assert_eq!(get_shortest_sequence(&"029A".to_string(), 2).len(), 68);
  }
  #[test]
  fn shortest_sequence_2() {
    assert_eq!(get_shortest_sequence(&"980A".to_string(), 2).len(), 60);
  }
  #[test]
  fn numeric_keypad_sequence() {
    assert_eq!(get_sequence_for_numeric_keypad(&"029A".to_string()), "<A^A>^^AvvvA");
  }
}
