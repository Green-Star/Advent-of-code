use std::{iter::{Enumerate, Rev}, ops::Index, result, slice::Iter};

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let _ = transform_data(data);

  let final_result = 0;

  println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> Vec<Option<i32>> {
  let mut result = vec![];

  let mut free_space = false;
  let mut file_id = 0;

  for line in data {
    for c in line.chars() {
      let length = c.to_digit(10).unwrap();

      for _ in 0..length {
        result.push({
          if free_space { None }
          else { Some(file_id) }
        });
      }

      match free_space {
          false => free_space = true,
          true => {
            free_space = false;
            file_id += 1;
          }
      }
    }
  }

  result
}
