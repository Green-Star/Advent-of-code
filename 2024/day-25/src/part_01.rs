use std::collections::HashMap;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  /* Believe it or not, but no transform_data today */

  let final_result: i64 = data.iter().map(|s| (get_numeric_code(s), get_shortest_sequence(s, 2))).map(|(code, sequnce)| code * (sequnce as i64)).sum();
  println!("Part 1 final result: {}", final_result);
}

type Lock = (usize, usize, usize, usize, usize);
type Key = (usize, usize, usize, usize, usize);

fn transform_data(data: Vec<String>) -> (Vec<Lock>, Vec<Key>) {
  let mut locks = vec![];
  let mut keys = vec![];

  let mut in_progress = false;
  let mut is_lock = false;
  let mut is_key = false;

  let mut pattern = vec![ vec![]; 5 ];

  for line in data {
    if line.is_empty() {
      if in_progress {
        if is_lock {
          locks.push(create_lock(&pattern));
        }
        if is_key {
          keys.push(create_key(&pattern));
        }
      }
      in_progress = false;
      is_lock = false;
      is_key = false;
      pattern = vec![ vec![]; 5 ];
    } else {
      /* First line of a new pattern: compute the pattern type (either '#####' -> Lock or '.....' -> Key) */
      if in_progress == false {
        if line == "#####" {
          is_lock = true;
        } else if line == "....." {
          is_key = true;
        } else {
          panic!("Unknown pattern!");
        }
        in_progress = true;
      }

      let mut chars = line.chars();

      pattern[0].push(chars.next().unwrap());
      pattern[1].push(chars.next().unwrap());
      pattern[2].push(chars.next().unwrap());
      pattern[3].push(chars.next().unwrap());
      pattern[4].push(chars.next().unwrap());
    }
  }
  /* Last pattern */
  if in_progress {
    if is_lock {
      locks.push(create_lock(&pattern));
    }
    if is_key {
      keys.push(create_key(&pattern));
    }
  }

  (locks, keys)
}
fn create_lock(lock: &Vec<Vec<char>>) -> Lock {
  let summary = lock.iter()
    .map(|chars| chars.iter().enumerate().rev().find_map(|(index, &c)| if c == '#' { Some(index) } else { None }))
    .map(|o| o.unwrap())
    .collect::<Vec<_>>();

  (summary[0], summary[1], summary[2], summary[3], summary[4])
}
fn create_key(key: &Vec<Vec<char>>) -> Key {
  let summary = key.iter()
    .map(|chars| chars.iter().rev().enumerate().rev().find_map(|(index, &c)| if c == '#' { Some(index) } else { None }))
    .map(|o| o.unwrap())
    .collect::<Vec<_>>();

  (summary[0], summary[1], summary[2], summary[3], summary[4])
}

fn get_numeric_code(code: &String) -> i64 {
  let mut s = code.chars();
  s.next_back();
  s.into_iter().map(|c| (c.to_digit(10).unwrap() as i64)).fold(0, |acc, x| acc * 10 + x)
}

fn get_shortest_sequence(code: &String, robots: u64) -> usize {
  get_sequences(code, robots).iter().map(|s| s.len()).min().unwrap()
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

fn get_sequence_for_numeric_keypad(code: &String) -> Vec<String> {
  Keypad::from(NumericKeypad::create_keypad()).get_sequence(code)
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
  fn test_transform_data() {
    let data = vec![
      "#####".to_string(),
      ".####".to_string(),
      ".####".to_string(),
      ".####".to_string(),
      ".#.#.".to_string(),
      ".#...".to_string(),
      ".....".to_string(),
      "".to_string(),
      "#####".to_string(),
      "##.##".to_string(),
      ".#.##".to_string(),
      "...##".to_string(),
      "...#.".to_string(),
      "...#.".to_string(),
      ".....".to_string(),
      "".to_string(),
      ".....".to_string(),
      "#....".to_string(),
      "#....".to_string(),
      "#...#".to_string(),
      "#.#.#".to_string(),
      "#.###".to_string(),
      "#####".to_string(),
      "".to_string(),
      ".....".to_string(),
      ".....".to_string(),
      "#.#..".to_string(),
      "###..".to_string(),
      "###.#".to_string(),
      "###.#".to_string(),
      "#####".to_string(),
      "".to_string(),
      ".....".to_string(),
      ".....".to_string(),
      ".....".to_string(),
      "#....".to_string(),
      "#.#..".to_string(),
      "#.#.#".to_string(),
      "#####".to_string(),
    ];

    let result = (
      vec![
        (0, 5, 3, 4, 3),
        (1, 2, 0, 5, 3),
      ],
      vec![
        (5, 0, 2, 1, 3),
        (4, 3, 4, 0, 2),
        (3, 0, 2, 0, 1),
      ],
    );

    assert_eq!(transform_data(data), result);
  }
  #[test]
  fn test_transfrom_lock() {
    let data = vec![
      "#####".to_string(),
      ".####".to_string(),
      ".####".to_string(),
      ".####".to_string(),
      ".#.#.".to_string(),
      ".#...".to_string(),
      ".....".to_string(),
    ];

    let result = (
      vec![ (0, 5, 3, 4, 3) ],
      vec![],
    );

    assert_eq!(transform_data(data), result);
  }
  #[test]
  fn test_transfrom_key() {
    let data = vec![
      ".....".to_string(),
      "#....".to_string(),
      "#....".to_string(),
      "#...#".to_string(),
      "#.#.#".to_string(),
      "#.###".to_string(),
      "#####".to_string(),
    ];

    let result = (
      vec![],
      vec![ (5, 0, 2, 1, 3) ],
    );

    assert_eq!(transform_data(data), result);
  }
}
