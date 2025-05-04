use std::collections::HashMap;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  /* Believe it or not, but no transform_data today */

  let final_result: i64 = data.iter().map(|s| (get_numeric_code(s), get_shortest_sequence(s, 2))).map(|(code, sequnce)| code * (sequnce as i64)).sum();
  println!("Part 1 final result: {}", final_result);
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
  let keypad = NumericKeypad::new();
  /* Always start on A */
  let full_code = "A".to_string() + code;

  let zipped = full_code.chars().into_iter().zip(code.chars().into_iter());
  zipped.into_iter()
        .try_fold(vec![ "".to_string() ], |acc, (from, to)| {
          keypad.get_paths(from, to)
                .map(|next_paths| next_paths.iter()
                                                        .flat_map(|path| acc.iter()
                                                                                    .map(|current_path| current_path.to_owned() + path)
                                                                                    .collect::<Vec<_>>()
                                                        )
                                                        .collect()
                )
        }).unwrap()
}

fn get_sequence_for_directionnal_keypad(sequence: &String) -> Vec<String> {
  let keypad = DirectionalKeypad::new();
  /* Always start on A */
  let full_code = "A".to_string() + sequence;

  let zipped = full_code.chars().into_iter().zip(sequence.chars().into_iter());
  zipped.into_iter()
        .try_fold(vec![ "".to_string() ], |acc, (from, to)| {
          keypad.get_paths(from, to)
                .map(|next_paths| next_paths.iter()
                                                        .flat_map(|path| acc.iter()
                                                                                    .map(|current_path| current_path.to_owned() + path)
                                                                                    .collect::<Vec<_>>()
                                                        )
                                                        .collect()
                )
        }).unwrap()
}


/*
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
  */
fn compute_next_value(s: &String) -> String {
  let pad = DirectionalKeypad::new();

    /*
  let keypad = DirectionalKeypad::create_directional_keypad();

  /* Robot sequences always start on A */
  let mut current = keypad.get(&'A').unwrap();

  let mut sequence = String::new();
  for c in s.chars() {
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
    if current.x == 0 && destination.y == 1 {
      sequence.push_str(&horizontal);
      sequence.push_str(&vertical);
    } else if current.y == 1 && destination.x == 0 {
      sequence.push_str(&vertical);
      sequence.push_str(&horizontal);
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
  */

  let keypad = DirectionalKeypad:: new();

  /* Robot sequences always start on A */
  let mut current = 'A';

  let mut sequence = String::new();
  for destination in s.chars() {

    /*
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+

    Direction priorities:
      1. Moving with least turns
      2. Then, inside a move:
          you always want to hit the far left key first (<), and then the middle key (^ or v) and then a right column key (> or A)
    */
    if current == 'A' {
      if destination == '^' { sequence.push_str("<") }
      else if destination == '<' { sequence.push_str("v<<") }
//      else if destination == '<' { sequence.push_str("<v<") }
      else if destination == 'v' { sequence.push_str("<v") }
      else if destination == '>' {sequence.push_str("v") }
    } else if current == '^' {
      if destination == 'A' { sequence.push_str(">") }
      else if destination == '<' { sequence.push_str("v<") }
      else if destination == 'v' { sequence.push_str("v") }
      else if destination == '>' { sequence.push_str("v>") }
    } else if current == '<' {
      if destination == '^' { sequence.push_str(">^") }
      else if destination == 'A' { sequence.push_str(">>^") }
      else if destination == 'v' { sequence.push_str(">") }
      else if destination == '>' { sequence.push_str(">>") }
    /*
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+

    Direction priorities:
      1. Moving with least turns
      2. Then, inside a move:
          you always want to hit the far left key first (<), and then the middle key (^ or v) and then a right column key (> or A)
    */
    } else if current == 'v' {
      if destination == '^' { sequence.push_str("^")  }
      else if destination == 'A' { sequence.push_str("^>") }
      else if destination == '<' { sequence.push_str("<") }
      else if destination == '>' { sequence.push_str(">") }
    } else if current == '>' {
      if destination == '^' { sequence.push_str("<^") }
      else if destination == 'A' { sequence.push_str("^") }
      else if destination == '<' { sequence.push_str("<<") }
      else if destination == 'v' { sequence.push_str("<") }
    }

    sequence.push('A');
    current = destination;
  }
  sequence
}




#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone)]
struct NumericKeypad {
  keypad: HashMap<Position, char>,
  map: HashMap<(char, char), Vec<String>>,
}
impl NumericKeypad {
  fn new() -> Self {
    let keypad =     HashMap::from([
      (Position { x: 0, y: 3 }, '7'), (Position { x: 1, y: 3 }, '8'), (Position { x: 2, y: 3 }, '9'),
      (Position { x: 0, y: 2 }, '4'), (Position { x: 1, y: 2 }, '5'), (Position { x: 2, y: 2 }, '6'),
      (Position { x: 0, y: 1 }, '1'), (Position { x: 1, y: 1 }, '2'), (Position { x: 2, y: 1 }, '3'),
                                      (Position { x: 1, y: 0 }, '0'), (Position { x: 2, y: 0 }, 'A'),
    ]);
    let map = create_map_from_keypad(&keypad);

    NumericKeypad { keypad, map }
  }
  fn get_paths(&self, from: char, to: char) -> Option<Vec<String>> {
    self.map.get(&(from, to)).cloned()
  }
}
/*
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
  */

/*
          +---+---+
  1:      | ^ | A |
      +---+---+---+
  0:  | < | v | > |
Y^    +---+---+---+
  X>    0   1   2
*/
#[derive(Debug, Clone)]
struct DirectionalKeypad {
  keypad: HashMap<Position, char>,
  map: HashMap<(char, char), Vec<String>>,
}
impl DirectionalKeypad {
  fn new() -> Self {
    let keypad = HashMap::from([
                                        (Position { x: 1, y: 1 }, '^'), (Position { x: 2, y: 1 }, 'A'),
        (Position { x: 0, y: 0 }, '<'), (Position { x: 1, y: 0 }, 'v'), (Position { x: 2, y: 0 }, '>'),
      ]);

    let map = create_map_from_keypad(&keypad);

    DirectionalKeypad { keypad, map }
  }
  fn get_paths(&self, from: char, to: char) -> Option<Vec<String>> {
    self.map.get(&(from, to)).cloned()
  }
}

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





/* OLD

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
  let pad = DirectionalKeypad::new();

    /*
  let keypad = DirectionalKeypad::create_directional_keypad();

  /* Robot sequences always start on A */
  let mut current = keypad.get(&'A').unwrap();

  let mut sequence = String::new();
  for c in s.chars() {
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
    if current.x == 0 && destination.y == 1 {
      sequence.push_str(&horizontal);
      sequence.push_str(&vertical);
    } else if current.y == 1 && destination.x == 0 {
      sequence.push_str(&vertical);
      sequence.push_str(&horizontal);
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
  */

  let keypad = DirectionalKeypad::create_directional_keypad();

  /* Robot sequences always start on A */
  let mut current = 'A';

  let mut sequence = String::new();
  for destination in s.chars() {

    /*
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+

    Direction priorities:
      1. Moving with least turns
      2. Then, inside a move:
          you always want to hit the far left key first (<), and then the middle key (^ or v) and then a right column key (> or A)
    */
    if current == 'A' {
      if destination == '^' { sequence.push_str("<") }
      else if destination == '<' { sequence.push_str("v<<") }
//      else if destination == '<' { sequence.push_str("<v<") }
      else if destination == 'v' { sequence.push_str("<v") }
      else if destination == '>' {sequence.push_str("v") }
    } else if current == '^' {
      if destination == 'A' { sequence.push_str(">") }
      else if destination == '<' { sequence.push_str("v<") }
      else if destination == 'v' { sequence.push_str("v") }
      else if destination == '>' { sequence.push_str("v>") }
    } else if current == '<' {
      if destination == '^' { sequence.push_str(">^") }
      else if destination == 'A' { sequence.push_str(">>^") }
      else if destination == 'v' { sequence.push_str(">") }
      else if destination == '>' { sequence.push_str(">>") }
    /*
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+

    Direction priorities:
      1. Moving with least turns
      2. Then, inside a move:
          you always want to hit the far left key first (<), and then the middle key (^ or v) and then a right column key (> or A)
    */
    } else if current == 'v' {
      if destination == '^' { sequence.push_str("^")  }
      else if destination == 'A' { sequence.push_str("^>") }
      else if destination == '<' { sequence.push_str("<") }
      else if destination == '>' { sequence.push_str(">") }
    } else if current == '>' {
      if destination == '^' { sequence.push_str("<^") }
      else if destination == 'A' { sequence.push_str("^") }
      else if destination == '<' { sequence.push_str("<<") }
      else if destination == 'v' { sequence.push_str("<") }
    }

    sequence.push('A');
    current = destination;
  }
  sequence
}




#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
struct DirectionalKeypad {
  keypad: HashMap<Position, char>,
  map: HashMap<(char, char), Vec<String>>,
}
impl DirectionalKeypad {
  fn new() -> Self {
    let keypad = HashMap::from([
                                        (Position { x: 1, y: 1 }, '^'), (Position { x: 2, y: 1 }, 'A'),
        (Position { x: 0, y: 0 }, '<'), (Position { x: 1, y: 0 }, 'v'), (Position { x: 2, y: 0 }, '>'),
      ]);

    let mut map = HashMap::new();

    for (&start_position, &start) in &keypad {
      for (&end_position, &end) in &keypad {
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
                                        'v' => (0, 1),
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

    DirectionalKeypad { keypad, map }
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
  fn test_sequence_1() {
    assert_eq!(get_shortest_sequence(&"029A".to_string(), 2), "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
  }
  #[test]
  fn test_sequence_2() {
    assert_eq!(get_shortest_sequence(&"980A".to_string(), 2), "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A");
  }
  #[test]
  fn test_sequence_3() {
    assert_eq!(get_shortest_sequence(&"179A".to_string(), 2), "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A");
  }
  #[test]
  fn test_sequence_4() {
    assert_eq!(get_shortest_sequence(&"456A".to_string(), 2), "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A");
  }
  #[test]
  fn test_sequence_5() {
    assert_eq!(get_shortest_sequence(&"379A".to_string(), 2), "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A");
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
  fn shortest_sequence_3() {
    assert_eq!(get_shortest_sequence(&"179A".to_string(), 2).len(), 68);
  }
  #[test]
  fn shortest_sequence_4() {
    assert_eq!(get_shortest_sequence(&"456A".to_string(), 2).len(), 64);
  }
  #[test]
  fn shortest_sequence_5() {
    assert_eq!(get_shortest_sequence(&"379A".to_string(), 2).len(), 64);
  }

  #[test]
  fn numeric_keypad_sequence() {
    assert_eq!(get_sequence_for_numeric_keypad(&"029A".to_string()), "<A^A>^^AvvvA");
  }
  #[test]
  fn numeric_and_directional_keypad_sequence_step_0() {
    assert_eq!(get_shortest_sequence(&"029A".to_string(), 0), "<A^A>^^AvvvA");
  }
  #[test]
  fn numeric_and_directional_keypad_sequence_step_1() {
    assert_eq!(get_shortest_sequence(&"029A".to_string(), 1), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A");
  }
  #[test]
  fn numeric_and_directional_keypad_sequence_step_2() {
    assert_eq!(get_shortest_sequence(&"029A".to_string(), 2), "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
  }
}
*/
