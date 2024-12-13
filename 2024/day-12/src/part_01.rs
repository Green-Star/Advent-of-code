use std::path::PrefixComponent;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let (mut map, empty_map) = transform_data(data);

  let final_result = do_stuff(map, &empty_map);

  println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> (Vec<Vec<Plot>>, Vec<Vec<Plot>>) {
  let mut result = vec![];
  let mut empty = vec![];

  for s in data {
    let mut line = vec![];
    let mut empty_line = vec![];
    for c in s.chars() {
      line.push(Plot { type_of: Some(c), connections: vec![] });
      empty_line.push(Plot { type_of: None, connections: vec![] });
    }
    empty.push(empty_line);
    result.push(line);
  }

  (result, empty)
}

fn do_stuff(mut map: Vec<Vec<Plot>>, empty_map: &Vec<Vec<Plot>>) -> u32 {
  let mut areas_sum = 0;

  print_map(&map);

  for i in 0..map.len() {
    for j in 0..map[i].len() {
      match map[i][j].type_of {
        None => {},
        Some(_) => {
          let mut extract_map = empty_map.clone();
          areas_sum += explore_area(&mut map, (i, j), &mut extract_map);
          print_map(&map);
          print_map(&extract_map);
        }
      }
    }
  }

  areas_sum
}

fn move_cell_at_position(src: &mut Vec<Vec<Plot>>, position: &(usize, usize), dst: &mut Vec<Vec<Plot>>) {
  dst[position.0][position.1].type_of = src[position.0][position.1].type_of;
  src[position.0][position.1].type_of = None;
}

fn explore_area(origin: &mut Vec<Vec<Plot>>, position: (usize, usize), empty_map: &mut Vec<Vec<Plot>>) -> u32 {
  move_cell_at_position(origin, &position, empty_map);
  move_area(origin, position, empty_map, &position);

  0
}

fn move_area(origin: &mut Vec<Vec<Plot>>, position: (usize, usize), empty_map: &mut Vec<Vec<Plot>>, ref_index: &(usize, usize)) {
  move_cells_in_area(origin, (position.0.checked_add_signed(-1), position.1.checked_add_signed(0)), empty_map, &ref_index);
  move_cells_in_area(origin, (position.0.checked_add_signed(0), position.1.checked_add_signed(1)), empty_map, &ref_index);
  move_cells_in_area(origin, (position.0.checked_add_signed(1), position.1.checked_add_signed(0)), empty_map, &ref_index);
  move_cells_in_area(origin, (position.0.checked_add_signed(0), position.1.checked_add_signed(-1)), empty_map, &ref_index);
}
fn move_cells_in_area(origin: &mut Vec<Vec<Plot>>, position: (Option<usize>, Option<usize>), empty_map: &mut Vec<Vec<Plot>>, ref_index: &(usize, usize)) {
  let x;
  let y;

  match position {
    (Some(i), Some(j)) => {
      if i >= origin.len() { return }
      if j >= origin[i].len() { return }
      x = i;
      y = j;
    },
    _ => return
  }

  if origin[x][y].type_of != empty_map[ref_index.0][ref_index.1].type_of { return }

  move_cell_at_position(origin, &(x, y), empty_map);
  move_area(origin, (x, y), empty_map, ref_index);
}


#[derive(Debug, Copy, Clone)]
enum Direction {
  North,
  East,
  South,
  West,
}

#[derive(Debug, Clone)]
struct Plot {
  type_of: Option<char>,
  connections: Vec<Direction>,
}
fn print_map(map: &Vec<Vec<Plot>>) {
  println!("***");
  for l in map {
    for p in l {
      match p.type_of {
        Some(x) => print!("[{x}]"),
        None => print!("[ ]"),
      }
    }
    println!();
  }
}

