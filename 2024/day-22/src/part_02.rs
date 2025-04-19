use std::collections::HashMap;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let secret_numbers = transform_data(data);

  let mut rainbow = SecretRainbow::new();
  let changes = get_all_changes(&mut rainbow, secret_numbers);

  let mut max = 0;

  for a in -9..=9 {
    println!("Testing {a}...");
    for b in -9..=9 {
      for c in -9..=9 {
        for d in -9..=9 {
          let bananas = get_sequence_value(&changes, &[a, b, c, d]);
          if bananas > max { max = bananas }
        }
      }
    }
  }

  let final_result = max;
  println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> Vec<i64> {
  let mut result = vec![];

  for line in data {
    result.push(line.parse().unwrap());
  }

  result
}

fn mix(a: i64, b: i64) -> i64 {
  a ^ b
}
fn prune(x: i64) -> i64 {
  x % 16777216
}

fn next_secret(secret: i64) -> i64 {
  let next = secret * 64;
  let secret = mix(secret, next);
  let secret = prune(secret);

  let next = secret / 32;
  let secret = mix(secret, next);
  let secret = prune(secret);

  let next = secret * 2048;
  let secret = mix(secret, next);
  let secret = prune(secret);

  secret
}

#[derive(Debug, Clone)]
struct Change {
  bananas: i64,
  change: i64,
}

fn get_changes_from_secret(rainbow: &mut SecretRainbow, start: i64) -> Vec<Change> {
  let mut changes = vec![];

  let mut secret = start;
  let mut previous_bananas = secret % 10;

  for _ in 2..=2000 {
    secret = rainbow.get_next_secret(secret);

    let bananas = secret % 10;
    let change = bananas - previous_bananas;

    changes.push(Change { bananas, change });

    previous_bananas = bananas;
  }

  changes
}

fn get_all_changes(rainbow: &mut SecretRainbow, secrets: Vec<i64>) -> Vec<Vec<Change>> {
  secrets.iter().map(|secret| get_changes_from_secret(rainbow, *secret)).collect()
}

fn get_sequence_value_in_change(change: &Vec<Change>, sequence: &[i64]) -> i64 {
  if let Some(slice) = change.windows(4)
                                        .find(|slice|
                                                slice[0].change == sequence[0] &&
                                                slice[1].change == sequence[1] &&
                                                slice[2].change == sequence[2] &&
                                                slice[3].change == sequence[3]) {
    slice[3].bananas
  } else {
    0
  }
}
fn get_sequence_value(changes: &Vec<Vec<Change>>, sequence: &[i64]) -> i64 {
  changes.iter().map(|change| get_sequence_value_in_change(change, sequence)).sum()
}

#[derive(Debug, Clone)]
struct SecretRainbow {
  secrets: HashMap<i64, i64>,
}
impl SecretRainbow {
  fn new() -> SecretRainbow {
    SecretRainbow { secrets: HashMap::new() }
  }

  fn get_next_secret(&mut self, secret: i64) -> i64 {
    if let Some(next) = self.secrets.get(&secret) {
      *next
    } else {
      let next = next_secret(secret);
      self.secrets.insert(secret, next);

      next
    }
  }

  fn get_final_secret(&mut self, secret: i64) -> i64 {
    let mut result = secret;
    for _ in 1..=2000 {
      result = self.get_next_secret(result);
    }
    result
  }
  fn get_all_final_secrets(&mut self, secrets: Vec<i64>) -> Vec<i64> {
    secrets.iter().map(|secret| self.get_final_secret(*secret)).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_mix() {
    assert_eq!(mix(42, 15), 37);
  }

  #[test]
  fn test_prune() {
    assert_eq!(prune(100000000), 16113920);
  }

  #[test]
  fn secret_1() {
    assert_eq!(next_secret(123), 15887950);
  }
  #[test]
  fn secret_2() {
    assert_eq!(next_secret(15887950), 16495136);
  }
  #[test]
  fn secret_3() {
    assert_eq!(next_secret(16495136), 527345);
  }
  #[test]
  fn secret_4() {
    assert_eq!(next_secret(527345), 704524);
  }
  #[test]
  fn secret_5() {
    assert_eq!(next_secret(704524), 1553684);
  }
  #[test]
  fn secret_6() {
    assert_eq!(next_secret(1553684), 12683156);
  }
  #[test]
  fn secret_7() {
    assert_eq!(next_secret(12683156), 11100544);
  }
  #[test]
  fn secret_8() {
    assert_eq!(next_secret(11100544), 12249484);
  }
  #[test]
  fn secret_9() {
    assert_eq!(next_secret(12249484), 7753432);
  }
  #[test]
  fn secret_10() {
    assert_eq!(next_secret(7753432), 5908254);
  }


  #[test]
  fn secret_rainbow() {
    let mut rainbow = SecretRainbow::new();

    assert_eq!(rainbow.get_next_secret(123), 15887950);
    assert_eq!(rainbow.get_next_secret(15887950), 16495136);
    assert_eq!(rainbow.get_next_secret(16495136), 527345);
    assert_eq!(rainbow.get_next_secret(527345), 704524);
    assert_eq!(rainbow.get_next_secret(704524), 1553684);
    assert_eq!(rainbow.get_next_secret(1553684), 12683156);
    assert_eq!(rainbow.get_next_secret(12683156), 11100544);
    assert_eq!(rainbow.get_next_secret(11100544), 12249484);
    assert_eq!(rainbow.get_next_secret(12249484), 7753432);
    assert_eq!(rainbow.get_next_secret(7753432), 5908254);
  }

  #[test]
  fn final_secret_rainbow() {
    let mut rainbow = SecretRainbow::new();

    assert_eq!(rainbow.get_final_secret(1), 8685429);
    assert_eq!(rainbow.get_final_secret(10), 4700978);
    assert_eq!(rainbow.get_final_secret(100), 15273692);
    assert_eq!(rainbow.get_final_secret(2024), 8667524);
  }
}
