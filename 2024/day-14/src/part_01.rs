pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let _ = transform_data(data);

  let final_result = 0;

  let mut r = Robot { start_position: (2, 4), velocity: (2, -3), end_position: (2, 4) };
  for j in 0..7 {
    for i in 0..11 {
      if i == r.end_position.0 && j == r.end_position.1 {
        print!("1");
      } else { print!(".");}
    }
    println!("");
  }
  println!("***");

  r.end_position = get_coordinates(&r, (11, 7), 0);
  for j in 0..7 {
    for i in 0..11 {
      if i == r.end_position.0 && j == r.end_position.1 {
        print!("1");
      } else { print!(".");}
    }
    println!("");
  }
  println!("***");

  r.end_position = get_coordinates(&r, (11, 7), 1);
  for j in 0..7 {
    for i in 0..11 {
      if i == r.end_position.0 && j == r.end_position.1 {
        print!("1");
      } else { print!(".");}
    }
    println!("");
  }
  println!("***");

  r.end_position = get_coordinates(&r, (11, 7), 2);
  for j in 0..7 {
    for i in 0..11 {
      if i == r.end_position.0 && j == r.end_position.1 {
        print!("1");
      } else { print!(".");}
    }
    println!("");
  }
  println!("***");

  r.end_position = get_coordinates(&r, (11, 7), 3);
  for j in 0..7 {
    for i in 0..11 {
      if i == r.end_position.0 && j == r.end_position.1 {
        print!("1");
      } else { print!(".");}
    }
    println!("");
  }
  println!("***");

  r.end_position = get_coordinates(&r, (11, 7), 4);
  for j in 0..7 {
    for i in 0..11 {
      if i == r.end_position.0 && j == r.end_position.1 {
        print!("1");
      } else { print!(".");}
    }
    println!("");
  }
  println!("***");

  r.end_position = get_coordinates(&r, (11, 7), 5);
  for j in 0..7 {
    for i in 0..11 {
      if i == r.end_position.0 && j == r.end_position.1 {
        print!("1");
      } else { print!(".");}
    }
    println!("");
  }
  println!("***");

  println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> Vec<i32> {
  let mut result = vec![];

  result
}

fn get_coordinates(robot: &Robot, map_size: (i32, i32), seconds: i32) -> (i32, i32) {
  let i_x = (robot.start_position.0 + robot.velocity.0 * seconds) % map_size.0;
  let i_y = (robot.start_position.1 + robot.velocity.1 * seconds) % map_size.1;

  let x = { if i_x < 0 { map_size.0 + i_x } else { i_x } };
  let y = { if i_y < 0 { map_size.1 + i_y } else { i_y } };

  (x, y)
}

#[derive(Debug, Copy, Clone)]
struct Robot {
  start_position: (i32, i32),
  velocity: (i32, i32),
  end_position: (i32, i32),
}

