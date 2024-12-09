use std::{ops::Index, result};

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let transformed_data = transform_data(data);
  println!("{:?}", transformed_data);
  println!("*****");
  let transformed_filesystem = compact_files(&transformed_data);
  println!("{:?}", transformed_filesystem);

  let final_result = checksum(&transformed_filesystem);

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

fn compact_files(filesystem: &Vec<Option<i32>>) -> Vec<Option<i32>> {
  let mut output = filesystem.clone();

  for (index, item) in filesystem.iter().enumerate().rev() {
    match item {
      None => {},
      Some(file_id) => {
        output[index] = None;
        match output.iter().position(|e| *e == None) {
          Some(index) => output[index] = Some(*file_id),
          None => break
        }
      }
    }
  }

  output
}

fn checksum(filesystem: &Vec<Option<i32>>) -> usize {
  filesystem.iter().enumerate().fold(0, |checksum, (index, item)| {
    match item {
      Some(file_id) => checksum + (index * (*file_id as usize)),
      None => checksum
    }
  })
}