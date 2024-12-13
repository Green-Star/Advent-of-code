use std::collections::HashMap;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let (mut city, antennas_list) = transform_data(data);
  let antenna_map = group_antennas(antennas_list);
  let paired_antennas = pair_antennas(&antenna_map);
  println!("{:?}", paired_antennas);

  for p in paired_antennas {
    let (a, b) = compute_antinode(&city, p);
    match a {
        Some((x, y)) => match city[x][y].antinode {
            None => city[x][y].antinode = Some(1),
            Some(_) => city[x][y].antinode = Some(1),
        },
        _ => {},
    }
    match b {
      Some((x, y)) => match city[x][y].antinode {
          None => city[x][y].antinode = Some(1),
          Some(_) => city[x][y].antinode = Some(1),
      },
      _ => {},
    }
  }

  let final_result = city.iter().map(|l| l.iter().fold(0, |sum, location| {
    match location.antinode {
        Some(x) => sum + x,
        None => sum,
    }
  })).sum::<i32>();

  println!("Part 1 final result: {}", final_result);
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
      }
      j += 1;
      chars.push(CityLocation { antinode: None });
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
fn compute_antinode(city: &Vec<Vec<CityLocation>>, paired_antennas: (Antenna, Antenna)) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
  let (a, b) = paired_antennas;
  let offset_a = ((a.position.0 as isize) - (b.position.0 as isize), (a.position.1 as isize) - (b.position.1 as isize));
  let offset_b = ((b.position.0 as isize) - (a.position.0 as isize), (b.position.1 as isize) - (a.position.1 as isize));

  println!("({:?} - {:?}) -> ({:?}, {:?})", a, b, offset_a, offset_b);

  let mut first_antinode_position = (a.position.0.checked_add_signed(offset_a.0), a.position.1.checked_add_signed(offset_a.1));
  match first_antinode_position {
      (Some(x), Some(y)) => {
        if x >= city.len() { first_antinode_position.0 = None }
        else if y >= city[x].len() { first_antinode_position.1 = None }
      },
      _ => {},
  }
  let mut second_antinode_position = (b.position.0.checked_add_signed(offset_b.0), b.position.1.checked_add_signed(offset_b.1));
  match second_antinode_position {
      (Some(x), Some(y)) => {
        if x >= city.len() { second_antinode_position.0 = None }
        else if y >= city[x].len() { second_antinode_position.1 = None }
      },
      _ => {},
  }

  let a_antinode_position;
  match first_antinode_position {
      (Some(x), Some(y)) => a_antinode_position = Some((x, y)),
      _ => a_antinode_position = None
  }
  let b_antinode_position;
  match second_antinode_position {
      (Some(x), Some(y)) => b_antinode_position = Some((x, y)),
      _ => b_antinode_position = None
  }

  (a_antinode_position, b_antinode_position)
}

#[derive(Debug, Copy, Clone)]
struct CityLocation {
  antinode: Option<i32>,
}