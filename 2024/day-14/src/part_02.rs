use std::collections::HashMap;
use bmp::{Image, Pixel};
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let map_size = (101, 103);
  let robots = transform_data(data);

  let mut entropies = vec![];

  println!("Generating images...");
  for seconds in 1..=9999 {
    if seconds % 1000 == 0 { println!("Image {seconds}/10000"); }

    let robots = robots.iter().map(|r| Robot { start_position: r.start_position, velocity: r.velocity, end_position: get_coordinates(r, map_size, seconds) }).collect();
    let map = build_robot_map(&robots);

    let mut img = Image::new(map_size.0 as u32, map_size.1 as u32);
    let mut pixel_data = vec![ bmp::consts::BLACK; (map_size.0 * map_size.1) as usize];

    for y in 0..map_size.1 {
      for x in 0..map_size.0 {
        let color = { if let Some(_) = map.get(&(x, y)) { bmp::consts::BLACK } else { bmp::consts::WHITE } };
        img.set_pixel(x as u32, y as u32, color);
        pixel_data[((map_size.1 - y - 1) * map_size.0 + x) as usize] = color;
      }
    }

    /* Saving image */
    let filename = format!("./images/{:04}.bmp", seconds);
    let _ = img.save(filename);

    /* Calculating image entropy to find an internal solution */
    let data = pixel_data_to_binary_data(&pixel_data);

    /* I use zlib encoding to compute the entropy for me (because I'm lazy and pretty bad at this kind of maths) */
    /* The less entropy, the better */
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&data).unwrap();
    let binary_data = e.finish().unwrap();
    let entropy = binary_data.len();

    entropies.push(Entropy { second: seconds, value: entropy });
  }

  entropies.sort_by(|a, b| a.value.cmp(&b.value));
  let final_result = entropies[0].second; /* Image 6493 */

  println!("Part 2 final result: {} (check the corresponding bmp in ./images folder to be sure)", final_result);
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


fn pixel_data_to_binary_data(data: &Vec<Pixel>) -> Vec<u8> {
  let mut final_data = vec![];
  for px in data {
    final_data.append(&mut vec![px.b, px.g, px.r]);
  }
  final_data
}
#[derive(Debug, Copy, Clone)]
struct Entropy {
  second: i32,
  value: usize,
}
