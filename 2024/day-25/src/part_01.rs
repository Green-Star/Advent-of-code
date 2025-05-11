pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let (locks, keys) = transform_data(data);

  let final_result = combine(&locks, &keys, 5);
  println!("Part 1 final result: {}", final_result.len());
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

fn combine(locks: &Vec<Lock>, keys: &Vec<Key>, pattern_size: usize) -> Vec<(Lock, Key)> {
  locks.iter().flat_map(|lock|
    keys.iter().filter(|key|
      lock.0 + key.0 <= pattern_size &&
      lock.1 + key.1 <= pattern_size &&
      lock.2 + key.2 <= pattern_size &&
      lock.3 + key.3 <= pattern_size &&
      lock.4 + key.4 <= pattern_size
    )
    .map(|key| (*lock, *key))
  )
  .collect::<Vec<_>>()
}

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
  #[test]
  fn test_combine() {
    let locks = vec![
      (0, 5, 3, 4, 3),
      (1, 2, 0, 5, 3),
    ];
    let keys =  vec![
      (5, 0, 2, 1, 3),
      (4, 3, 4, 0, 2),
      (3, 0, 2, 0, 1),
    ];

    let result = vec![
      ((0, 5, 3, 4, 3), (3, 0, 2, 0, 1)),
      ((1, 2, 0, 5, 3), (4, 3, 4, 0, 2)),
      ((1, 2, 0, 5, 3), (3, 0, 2, 0, 1)),
    ];

    assert_eq!(combine(&locks, &keys, 5), result);
  }
}
