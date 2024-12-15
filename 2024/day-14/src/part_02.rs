use std::collections::HashMap;
use bmp::Image;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let map_size = (101, 103);
  let robots = transform_data(data);

  let mut seconds = 231;

  while seconds < 9999 {
    seconds += 101; /* Found a vertical pattern every 101 images (probably has something to do with the map size), starting at second 231 */

//  for seconds in 1..=9999 {
    let robots = robots.iter().map(|r| Robot { start_position: r.start_position, velocity: r.velocity, end_position: get_coordinates(r, map_size, seconds) }).collect();
    let map = build_robot_map(&robots);

    let mut img = Image::new(map_size.0 as u32, map_size.1 as u32);
    for y in 0..map_size.1 {
      for x in 0..map_size.0 {
        match map.get(&(x, y)) {
          Some(_) => img.set_pixel(x as u32, y as u32, bmp::consts::BLACK),
          None => img.set_pixel(x as u32, y as u32, bmp::consts::WHITE),
        }
      }
    }
    let filename = format!("./extracted-images/{:04}.bmp", seconds);
    let _ = img.save(filename);
  }

  let final_result = "WTF?"; /* Image 6493 */

  println!("Part 2 final result: {}", final_result);
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

#[derive(Debug, Copy, Clone)]
struct Robot {
  start_position: (i32, i32),
  velocity: (i32, i32),
  end_position: (i32, i32),
}

