use std::collections::HashMap;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  /* Believe it or not, but no transform_data today */

  let final_result: u128 = data.iter().map(|s| (get_numeric_code(s), get_shortest_sequence(s, 25))).map(|(code, sequence)| (code as u128) * (sequence as u128)).sum();
  println!("Part 2 final result: {}", final_result);
}

fn get_numeric_code(code: &String) -> i64 {
  let mut s = code.chars();
  s.next_back();
  s.into_iter().map(|c| (c.to_digit(10).unwrap() as i64)).fold(0, |acc, x| acc * 10 + x)
}

fn get_shortest_sequence(code: &String, robots: u64) -> usize {
  let mut cache: HashMap<(u64, char, char), usize> = HashMap::new();
  let numeric_keypad = Keypad::from(NumericKeypad::create_keypad());

  /* Always start on A */
  let full_code = "A".to_string() + code;
  let zipped = full_code.chars().into_iter().zip(code.chars().into_iter());
  zipped.into_iter()
        .fold(0, |acc, (from, to)| acc + get_min_length_sequence(0, from, to, robots, &numeric_keypad, &mut cache))
}

fn get_sequences(code: &String, robots: u64) -> Vec<String> {
  let mut sequence = get_sequence_for_numeric_keypad(code);

  for _ in 1..=robots {
    let mut next = vec![];
    for s in sequence {
      next.push(get_sequence_for_directionnal_keypad(&s));
    }
    sequence = next.iter().flatten().map(|s| s.to_owned()).collect();
  }

  sequence
}

fn get_min_length_sequence(level: u64, from: char, to: char, max_level: u64, keypad: &Keypad, cache: &mut HashMap<(u64, char, char), usize>) -> usize {
  if let Some(&min) = cache.get(&(level, from, to)) { return min }

//  println!("Level:{level}, ({from}->{to})");
//  println!("Level:{level}, ({from}->{to}) => [{:?}]", keypad.get_paths(from, to));
  let directional_keypad = Keypad::from(DirectionalKeypad::create_keypad());

  let shortest_sequence = keypad.get_paths(from, to)
                                      .iter()
                                      .map(|paths| {
  //                                      println!("Level:{level}, ({from}->{to})");
                                        if level == max_level { return paths.iter().map(|s| s.len()).min().unwrap() }

                                        paths.iter().map(|path| {
                                          let full_path = "A".to_string() + path;
                                          let zipped = full_path.chars().into_iter().zip(path.chars().into_iter());
                                          zipped.fold(0, |acc, (from, to)| acc + get_min_length_sequence(level + 1, from, to, max_level, &directional_keypad, cache))
                                        })
                                        .min()
                                        .unwrap()
                                      })
                                      .min()
                                      .unwrap();

  cache.insert((level, from, to), shortest_sequence);
  println!("Shortest sequence for ({from}->{to} at level {level}) is {shortest_sequence}");
  shortest_sequence
}

fn get_sequence_for_numeric_keypad(code: &String) -> Vec<String> {
  Keypad::from(NumericKeypad::create_keypad()).get_complete_sequence(code)
}

fn get_sequence_for_directionnal_keypad(sequence: &String) -> Vec<String> {
  Keypad::from(DirectionalKeypad::create_keypad()).get_sequence(sequence)
}



#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
  x: usize,
  y: usize,
}

#[derive(Debug, Clone)]
struct Keypad {
  _keypad: HashMap<Position, char>,
  map: HashMap<(char, char), Vec<String>>,
}
impl Keypad {
  fn create_map_from_keypad(keypad: &HashMap<Position, char>) -> HashMap<(char, char), Vec<String>> {
    let mut map = HashMap::new();

    for (&start_position, &start) in keypad {
      for (&end_position, &end) in keypad {
        if start == end { map.insert((start, end), vec![ "A".to_string() ]); continue; }

        let offset_x = start_position.x.abs_diff(end_position.x);
        let offset_y = start_position.y.abs_diff(end_position.y);

        let mut horizontal = vec![];
        for _ in 1..=offset_x {
          if end_position.x < start_position.x {
            horizontal.push('<');
          } else {
            horizontal.push('>');
          }
        }
        let mut vertical = vec![];
        for _ in 1..=offset_y {
          if end_position.y < start_position.y {
            vertical.push('v');
          } else {
            vertical.push('^');
          }
        }

        let mut possible_paths = vec![];
        if horizontal.is_empty() == false && vertical.is_empty() == false {
          possible_paths.push([horizontal.clone(), vertical.clone()].concat());
          possible_paths.push([vertical.clone(), horizontal.clone()].concat());
        } else if horizontal.is_empty() == false {
          possible_paths.append(&mut vec![ horizontal ]);
        } else if vertical.is_empty() == false {
          possible_paths.append(&mut vec![ vertical ]);
        }

        let mut paths = possible_paths.iter()
                                  .filter(|path| {
                                    path.iter().try_fold(start_position, |current_pos, direction| {
                                      let offset = match direction {
                                        '^' => (0, 1),
                                        '<' => (-1, 0),
                                        'v' => (0, -1),
                                        '>' => (1, 0),
                                        _ => panic!("Unknown direction"),
                                      };
                                      let next = Position { x: current_pos.x.checked_add_signed(offset.0).unwrap(), y: current_pos.y.checked_add_signed(offset.1).unwrap() };
                                      keypad.get(&next).map(|_| next)
                                    }).is_some()
                                  })
                                  .map(|path| String::from_iter(path.iter()))
                                  .collect::<Vec<_>>();
        paths.iter_mut().for_each(|s| s.push('A'));

        map.insert((start, end), paths);
      }
    }

    map
  }

  fn from(keypad: HashMap<Position, char>) -> Self {
    let map = Keypad::create_map_from_keypad(&keypad);

    Keypad { _keypad: keypad, map }
  }
  fn get_paths(&self, from: char, to: char) -> Option<Vec<String>> {
    self.map.get(&(from, to)).cloned()
  }
  fn get_complete_sequence(&self, sequence: &String) -> Vec<String> {
    /* Always start on A */
    let full_code = "A".to_string() + sequence;

    let zipped = full_code.chars().into_iter().zip(sequence.chars().into_iter());
    zipped.into_iter()
          .try_fold(vec![ "".to_string() ], |acc, (from, to)| {
            self.get_paths(from, to)
                .map(|next_paths| next_paths.iter()
                                                        .flat_map(|path| acc.iter()
                                                                                    .map(|current_path| current_path.to_owned() + path)
                                                                                    .collect::<Vec<_>>()
                                                        )
                                                        .collect()
                )
          }).unwrap()
  }

  fn get_sequence(&self, sequence: &String) -> Vec<String> {
    /* Always start on A */
    let full_code = "A".to_string() + sequence;

    let zipped = full_code.chars().into_iter().zip(sequence.chars().into_iter());
    zipped.into_iter()
          .try_fold(vec![ "".to_string() ], |acc, (from, to)| {
            self.get_paths(from, to)
                .map(|next_paths| next_paths.iter()
                                                        .flat_map(|path| acc.iter()
                                                                                    .map(|current_path| current_path.to_owned() + path)
                                                                                    .collect::<Vec<_>>()
                                                        )
                                                        .collect()
                )
          }).unwrap()
  }

  fn get_shortest_sequence_for_char(&self, from: char, to: char, level: usize, max_level: usize, cache: &mut HashMap<(usize, char, char), usize>) -> usize {
    if let Some(min) = cache.get(&(level, from, to)) { return *min }

    let min_length = self.get_paths(from, to)
                                              .iter()
                                              .map(|paths| {
                                                if level == max_level { return paths[0].len() }

                                                self.get_shortest_sequence_for_char(from, to, level + 1, max_level, cache)

                                              })
                                              .min()
                                              .unwrap();

    cache.insert((level, from, to), min_length);
    min_length
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
impl DirectionalKeypad {
  fn create_keypad() -> HashMap<Position, char> {
    HashMap::from([
      (Position { x: 1, y: 1 }, '^'), (Position { x: 2, y: 1 }, 'A'),
      (Position { x: 0, y: 0 }, '<'), (Position { x: 1, y: 0 }, 'v'), (Position { x: 2, y: 0 }, '>'),
    ])
  }
}
struct DirectionalKeypad {}
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
impl NumericKeypad {
  fn create_keypad() -> HashMap<Position, char> {
    HashMap::from([
      (Position { x: 0, y: 3 }, '7'), (Position { x: 1, y: 3 }, '8'), (Position { x: 2, y: 3 }, '9'),
      (Position { x: 0, y: 2 }, '4'), (Position { x: 1, y: 2 }, '5'), (Position { x: 2, y: 2 }, '6'),
      (Position { x: 0, y: 1 }, '1'), (Position { x: 1, y: 1 }, '2'), (Position { x: 2, y: 1 }, '3'),
                                      (Position { x: 1, y: 0 }, '0'), (Position { x: 2, y: 0 }, 'A'),
    ])
  }
}
struct NumericKeypad {}

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
  }
  #[test]
  fn test_extract_code_3() {
    assert_eq!(get_numeric_code(&"179A".to_string()), 179);
  }
  #[test]
  fn test_extract_code_4() {
    assert_eq!(get_numeric_code(&"456A".to_string()), 456);
  }
  #[test]
  fn test_extract_code_5() {
    assert_eq!(get_numeric_code(&"379A".to_string()), 379);
  }

  #[test]
  fn shortest_sequence_1() {
    assert_eq!(get_shortest_sequence(&"029A".to_string(), 2), 68);
  }
  #[test]
  fn shortest_sequence_2() {
    assert_eq!(get_shortest_sequence(&"980A".to_string(), 2), 60);
  }
  #[test]
  fn shortest_sequence_3() {
    assert_eq!(get_shortest_sequence(&"179A".to_string(), 2), 68);
  }
  #[test]
  fn shortest_sequence_4() {
    assert_eq!(get_shortest_sequence(&"456A".to_string(), 2), 64);
  }
  #[test]
  fn shortest_sequence_5() {
    assert_eq!(get_shortest_sequence(&"379A".to_string(), 2), 64);
  }

  #[test]
  fn numeric_keypad_sequence() {
    assert!(get_sequence_for_numeric_keypad(&"029A".to_string()).contains(&"<A^A>^^AvvvA".to_string()));
  }
  #[test]
  fn numeric_and_directional_keypad_sequence_step_0() {
    assert!(get_sequences(&"029A".to_string(), 0).contains(&"<A^A>^^AvvvA".to_string()));
  }
  #[test]
  fn numeric_and_directional_keypad_sequence_step_1() {
    assert!(get_sequences(&"029A".to_string(), 1).contains(&"v<<A>>^A<A>AvA<^AA>A<vAAA>^A".to_string()));
  }
  #[test]
  fn numeric_and_directional_keypad_sequence_step_2() {
    assert!(get_sequences(&"029A".to_string(), 2).contains(&"<vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A".to_string()));
  }
}
