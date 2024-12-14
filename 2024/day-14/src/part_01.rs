use std::{collections::HashMap};

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let map_size = (11, 7);
  let seconds = 100;
  let robots = transform_data(data);
/*
  let map = build_robot_map(&robots);
  for j in 0..7 {
    for i in 0..11 {
      match map.get(&(i, j)) {
        Some(number) => print!("{number}"),
        None => print!(".")
      }
    }
    println!("");
  }
  println!("***");
*/
  let robots = robots.iter().map(|r| Robot { start_position: r.start_position, velocity: r.velocity, end_position: get_coordinates(r, map_size, seconds) }).collect();
  let map = build_robot_map(&robots);
/*
  for j in 0..7 {
    for i in 0..11 {
      match map.get(&(i, j)) {
        Some(number) => print!("{number}"),
        None => print!(".")
      }
    }
    println!("");
  }
  println!("***");
  */

  let (north_west, north_east, south_west, south_east) = split_robot_map_in_quadrant(map, map_size);
  println!("({}, {}, {}, {})", north_west, north_east, south_west, south_east);

  let final_result = north_west * north_east * south_west * south_east;

  println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> Vec<Robot> {
  let mut result = vec![];

  for line in data {
    let mut s = line.split(" ");
    let (p, v) = (s.next().unwrap(), s.last().unwrap());
    let position = extract_numbers(p);
    let velocity = extract_numbers(v);
    result.push(Robot { start_position: position, velocity, end_position: position });
  }

  result
}
fn extract_numbers(chars: &str) -> (i32, i32) {
  let number_list = chars.split("=").last().unwrap();
  let mut numbers = number_list.split(",");
  let (x, y) = (numbers.next().unwrap().parse().unwrap(), numbers.last().unwrap().parse().unwrap());

  (x, y)
}

fn get_coordinates(robot: &Robot, map_size: (i32, i32), seconds: i32) -> (i32, i32) {
  let i_x = (robot.start_position.0 + robot.velocity.0 * seconds) % map_size.0;
  let i_y = (robot.start_position.1 + robot.velocity.1 * seconds) % map_size.1;

  let x = { if i_x < 0 { map_size.0 + i_x } else { i_x } };
  let y = { if i_y < 0 { map_size.1 + i_y } else { i_y } };

  (x, y)
}

fn build_robot_map(robots: &Vec<Robot>) -> HashMap<(i32, i32), i32> {
  let mut map = HashMap::new();

  for r in robots {
    map.entry(r.end_position).and_modify(|e| *e += 1).or_insert(1);
  }

  map
}
fn split_robot_map_in_quadrant(robot_map: HashMap<(i32, i32), i32>, map_size: (i32, i32)) -> (i32, i32, i32, i32) {
  let mut north_west = 0;
  let mut north_east = 0;
  let mut south_west = 0;
  let mut south_east = 0;

  let split_x = map_size.0 / 2;
  let split_y = map_size.1 / 2;

  for i in 0..split_x {
    for j in 0..split_y {
      match robot_map.get(&(i, j)) {
        Some(number) => north_west += number,
        None => {},
      }
    }
  }
  for i in split_x + 1..map_size.0 {
    for j in 0..split_y {
      match robot_map.get(&(i, j)) {
        Some(number) => north_east += number,
        None => {},
      }
    }
  }
  for i in 0..split_x {
    for j in split_y + 1..map_size.1 {
      match robot_map.get(&(i, j)) {
        Some(number) => south_west += number,
        None => {},
      }
    }
  }
  for i in split_x + 1..map_size.0 {
    for j in split_y + 1..map_size.1 {
      match robot_map.get(&(i, j)) {
        Some(number) => south_east += number,
        None => {},
      }
    }
  }

  (north_west, north_east, south_west, south_east)
}

#[derive(Debug, Copy, Clone)]
struct Robot {
  start_position: (i32, i32),
  velocity: (i32, i32),
  end_position: (i32, i32),
}

