pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let transformed_data = transform_data(data);

  let transformed_filesystem = compact_files(&transformed_data);

  let final_result = checksum(&transformed_filesystem);

  println!("Part 2 final result: {}", final_result);
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
  let mut last_read_index = output.len()-1;

  loop {
    match find_next_block_to_move(&output, last_read_index) {
      None => break,
      Some((block_index, block_length)) => {
        match find_free_chunk(&output, block_index, block_length) {
          Some(destination_index) => {
            for i in 0..block_length {
              output[destination_index + i] = output[block_index + i];
              output[block_index + i] = None;
            }
          },
          None => {},
        }
        match block_index.checked_add_signed(-1) {
          None => break,
          Some(next_index) => last_read_index = next_index,
        }
      }
    }
  }

  output
}

fn find_next_block_to_move(filesystem: &Vec<Option<i32>>, end_index: usize) -> Option<(usize, usize)> {
  let mut end_block_index = 0;
  let mut file_id = None;

  for i in (0..=end_index).rev() {
    match filesystem[i] {
      None => {},
      Some(file) => {
        end_block_index = i;
        file_id = Some(file);
        break;
      }
    }
  }
  if file_id == None { return None }

  let mut start_block_index = end_block_index;
  for i in (0..end_block_index).rev() {
    match filesystem[i] {
      Some(x) => {
        if Some(x) == file_id {
          start_block_index = i;
        } else {
          break;
        }
      },
      None => break,
    }
  }

  Some((start_block_index, end_block_index - start_block_index + 1))
}
fn find_free_chunk(filesystem: &Vec<Option<i32>>, block_index: usize, block_length: usize) -> Option<usize> {
  let mut read_index = 0;
  while read_index < block_index {
    match filesystem[read_index] {
      Some(_) => { read_index += 1 },
      None => {
        let mut chunk_length = 0;
        let mut i = 0;

        while read_index+i < block_index {
          match filesystem[read_index+i] {
            Some(_) => { break; },
            None => {
              chunk_length += 1;
              if chunk_length == block_length { return Some(read_index); }
              i += 1
            }
          }
        }
        read_index = read_index+i+1;
      }
    }
  }

  None
}

fn checksum(filesystem: &Vec<Option<i32>>) -> usize {
  filesystem.iter().enumerate().fold(0, |checksum, (index, item)| {
    match item {
      Some(file_id) => checksum + (index * (*file_id as usize)),
      None => checksum
    }
  })
}
