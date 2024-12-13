use std::collections::HashMap;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let (mut city, antennas_list) = transform_data(data);
  let antenna_map = group_antennas(antennas_list);
  let paired_antennas = pair_antennas(&antenna_map);

  for p in paired_antennas {
    let antinodes = compute_antinode(&city, p);

    for (x, y) in antinodes {
      match city[x][y].antinode {
        None => city[x][y].antinode = Some(1),
        Some(n) => city[x][y].antinode = Some(n + 1),
      }
    }
  }

  let final_result = city.iter().map(|l| l.iter().fold(0, |sum, location| {
    match location.antinode {
        Some(_) => sum + 1,
        None => sum,
    }
  })).sum::<i32>();

  println!("Part 2 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> (Vec<Vec<CityLocation>>, Vec<Antenna>) {
  let mut city = Vec::new();
  let mut antennas = Vec::new();

  for i in 0..data.len() {
    let mut chars = Vec::new();
    let mut j = 0;
    for c in data[i].chars() {
      if c.is_digit(36) {
        antennas.push(Antenna { frequency: c, position: (i, j) });
        chars.push(CityLocation { antinode: Some(1) }); // There's always at least 3 antennas for each frequency, so each antenna is itself an antinode (since it it aligned with at least two other antennas on its frequency)
      } else {
        chars.push(CityLocation { antinode: None });
      }
      j += 1;
    }
    city.push(chars);
  }

  (city, antennas)
}

#[derive(Debug, Copy, Clone)]
struct Antenna {
  frequency: char,
  position: (usize, usize),
}

fn group_antennas(list: Vec<Antenna>) -> HashMap<char, Vec<Antenna>> {
  let mut result = HashMap::new();

  for a in list {
    result.entry(a.frequency).and_modify(|e: &mut Vec<Antenna>| e.push(a)).or_insert(vec![a]);
  }

  result
}
fn pair_antennas(antenna_map: &HashMap<char, Vec<Antenna>>) -> Vec<(Antenna, Antenna)> {
  let mut result = Vec::new();

  for v in antenna_map.values() {
    for i in 0..v.len() {
      for j in i+1..v.len() {
        result.push((v[i], v[j]));
      }
    }
  }

  result
}
/* Create a vector of position (cf un peu comme en dessous) */
fn compute_antinode(city: &Vec<Vec<CityLocation>>, paired_antennas: (Antenna, Antenna)) -> Vec<(usize, usize)> {
  let mut result = vec![];

  let (mut a, mut b) = paired_antennas;
  let offset_a = ((a.position.0 as isize) - (b.position.0 as isize), (a.position.1 as isize) - (b.position.1 as isize));
  let offset_b = ((b.position.0 as isize) - (a.position.0 as isize), (b.position.1 as isize) - (a.position.1 as isize));

  loop {
    let antinode_position = (a.position.0.checked_add_signed(offset_a.0), a.position.1.checked_add_signed(offset_a.1));
    match antinode_position {
        (Some(x), Some(y)) => {
          if x >= city.len() { break }
          if y >= city[x].len() { break }
          result.push((x, y));
          a = Antenna { frequency: a.frequency, position: (x, y) };
        },
        _ => break,
    }
  }
  loop {
    let antinode_position = (b.position.0.checked_add_signed(offset_b.0), b.position.1.checked_add_signed(offset_b.1));
    match antinode_position {
        (Some(x), Some(y)) => {
          if x >= city.len() { break }
          if y >= city[x].len() { break }
          result.push((x, y));
          b = Antenna { frequency: b.frequency, position: (x, y) };
        },
        _ => break,
    }
  }

  result
}

#[derive(Debug, Copy, Clone)]
struct CityLocation {
  antinode: Option<i32>,
}