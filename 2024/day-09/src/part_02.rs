use std::{iter::{Enumerate, Rev}, ops::Index, result, slice::Iter};

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let transformed_data = transform_data(data);
  println!("{:?}", transformed_data);
  println!("*****");
  let transformed_filesystem = compact_files(&transformed_data);
  println!("finished!");
  println!("{:?}", transformed_filesystem);

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

  let mut i = 0;
  loop {
    match find_next_block_to_move(&output, last_read_index) {
      None => break,
      Some((block_index, block_length)) => {
        println!("Found block @{} -> {} cases", block_index, block_length);
        match find_free_chunk(&output, block_index, block_length) {
          Some(destination_index) => {
            println!("Move to {destination_index}");
            for i in 0..block_length {
              output[destination_index + i] = output[block_index + i];
              output[block_index + i] = None;
            }
          },
          None => {},
        }
//        println!("{:?}", output);
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
//        println!("{read_index} - {:?}", filesystem[read_index]);
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
/*
fn find_next_free_block(filesystem: &Vec<Option<i32>>, start_index: usize, end_index: usize) -> Option<usize> {
  for i in start_index..=end_index {
    match filesystem[i] {
      None => { return Some(i); },
      Some(_) => {}
    }
  }
  None
}
fn find_next_free_chunk(filesystem: &Vec<Option<i32>>, start_index: usize, end_index: usize, block_length: usize) -> Option<usize> {
  let mut length = 0;
  for i in start_index..=end_index {
    match filesystem[i] {
      None => {
        length = 0;
        for j in i..=end_index {
          match filesystem[j] {
            None => {
              length += 1;
              if length == block_length { return Some(i); }
            }
            Some(_) => { break; }
          }
        }
      },
      Some(_) => {}
    }
  }
  None
}
fn find_next_block_to_move(filesystem: &Vec<Option<i32>>, start_index: usize) -> Option<(usize, usize)> {
  let mut block_index = None;

  for i in (0..start_index).rev() {
    match filesystem[i] {
        None => {},
        Some(_) => {
          block_index = Some(i);
          break;
        }
    }
  }
  if block_index == None { return None; }

  let end_index = block_index.unwrap();
  let mut start_index = end_index;

  for i in (0..=end_index).rev() {
    match filesystem[i] {
      None => break,
      Some(_) => {
        if filesystem[i] != filesystem[end_index] { break }
        start_index = i;
      }
    }
  }

  Some((start_index, end_index - start_index + 1))
}

/*
fn compact_files(filesystem: &Vec<Option<i32>>) -> Vec<Option<i32>> {
  let mut output = filesystem.clone();
  let mut slice_start_index = 0;
  let mut end_slice_index = output.len();

  loop {
      match find_next_block_to_move(&output[slice_start_index..end_slice_index]) {
        None => break,
        Some((block_index, block_length)) => {
          println!("Trying to move {:?}", output[block_index]);
          match find_next_free_chunk(&output[slice_start_index..block_index], block_length) {
            None => { println!("Can't move!"); },
            Some(chunk_index) => {
              for i in 0..block_length {
                output[slice_start_index + chunk_index + i] = output[block_index + i];
                output[block_index + i] = None;
              }
              match find_next_free_block(&output[..block_index]) {
                None => break,
                Some(free_index) => slice_start_index += free_index,
              }
            }
          }
          println!("Searching in: {:?}", &output[slice_start_index..end_slice_index]);
          println!("block index {:?}", block_index);
          end_slice_index = block_index;
          println!("Searching in: {:?}", &output[slice_start_index..end_slice_index]);
        }
      }
  }

  output
}
  */

  /*
fn find_next_block_to_move(filesystem: &[Option<i32>]) -> Option<(usize, usize)> {
  let mut block_index = None;

  for i in (0..filesystem.len()).rev() {
    match filesystem[i] {
        None => {},
        Some(_) => {
          block_index = Some(i);
          break;
        }
    }
  }
  if block_index == None { return None; }

  let end_index = block_index.unwrap();
  let mut start_index = end_index;

  for i in (0..=end_index).rev() {
    match filesystem[i] {
      None => break,
      Some(_) => {
        if filesystem[i] != filesystem[end_index] { break }
        start_index = i;
      }
    }
  }

  Some((start_index, end_index - start_index + 1))
}
  */
  /***
fn find_next_free_chunk(filesystem: &[Option<i32>], chunk_length: usize) -> Option<usize> {
  let mut sliced_index = 0;

  loop {
    let free = find_next_free_block(&filesystem[sliced_index..]);
    match free {
      None => { break }
      Some(offset_free_index) => {
        let free_index = sliced_index + offset_free_index;

        let mut free_length = 0;
        for i in free_index ..= filesystem.len() {
          if i == filesystem.len() { return None }
          match filesystem[i] {
            Some(_) => {
              sliced_index = i;
              break;
            },
            None => {
              free_length += 1;
              if free_length == chunk_length { return Some(free_index); }
            }
          }
        }
      }
    }
  }

  None
}
fn find_next_free_block(filesystem: &[Option<i32>]) -> Option<usize> {
  filesystem.iter().position(|b| b == &None)
}
***/
/*



fn compact_files(filesystem: &Vec<Option<i32>>) -> Vec<Option<i32>> {
  let mut output = filesystem.clone();
/*

  let mut free_index = 0;

  let mut reversed_filesystem = filesystem.iter().enumerate().rev();

  for (index, item) in reversed_filesystem.next() {
    let move_index = find_next_free_chunk(&(output[free_index..index]), 2);
    println!("{:?}", move_index);

    let move_index = 2;
    free_index = move_index + 2;

    /* */
    reversed_filesystem.next();

    let (index, item) = reversed_filesystem.next() ;
    let move_index = find_next_free_chunk(&(output[free_index..index]), 2);

  }carg
*/
  let mut start_index = 0;
  let mut end_index = output.len();

  loop {
    println!("*** Output:  ***");
    println!("{:?}", &output[start_index..end_index]);

      match find_next_block(&output[start_index..end_index]) {
        None => break,
        Some((block_index, block_length)) => {
          println!("Block found: {}on{}blocks", block_index, block_length);
          match find_next_free_chunk(&output[start_index..start_index + block_index], block_length) {
            None => { println!("Can't move!"); },
            Some(new_index) => {
              println!("Find new index at {}", new_index);
              for i in 0..block_length {
                output[start_index + new_index + i] = output[block_index + i];
                output[block_index + i] = None;
              }
            }
          }
          end_index = block_index;
//          start_index = start_index + find_next_free_block(&output[0..end_index]).unwrap_or(end_index);
          println!("New indexes: {}->{}", start_index, end_index)
        }
      }
  }

  output
}
fn find_next_block(filesystem: &[Option<i32>]) -> Option<(usize, usize)> {
  let mut file = None;

  let mut end_index = filesystem.len()-1;
  for i in (0..filesystem.len()).rev() {
    println!("{} - {:?}", i, filesystem[i]);
    match filesystem[i] {
      None => {},
      Some(_) => {
        end_index = i;
        file = filesystem[i];
        break;
      }
    }
  }
  if file == None { return None; }

  let mut start_index = end_index;
  for i in (0..=end_index).rev() {
    match filesystem[i] {
      Some(_) => {
        if filesystem[i] == file { start_index = i }
        else { break }
      },
      None => break,
    }
  }

  Some((start_index, end_index - start_index + 1))
}

fn find_next_free_block(filesystem: &[Option<i32>]) -> Option<usize> {
  filesystem.iter().position(|b| b == &None)
}
fn find_next_free_chunk(filesystem: &[Option<i32>], chunk_length: usize) -> Option<usize> {
  println!("Chunk: {:?}", filesystem);
  match find_next_free_block(filesystem) {
    None => { None },
    Some(index) => {
      println!("Next free index {}", index);
      for i in 1..chunk_length {
        if index + i >= filesystem.len() { return None }
        match filesystem[index + i] {
          Some(_) => { return find_next_free_chunk(&filesystem[index + i..], chunk_length) },
          None => {},
        }
      }
      Some(index)
    }
  }
}
/*
fn find_next_block(reversed_filesystem: &mut Rev<Enumerate<Iter<'_, Option<i32>>>>) -> Option<(usize, usize)> {
  /*
  loop {
    match reversed_filesystem.next() {
      None => { return None },
      Some(option) =>
    }
  }
  */
  let mut length = 0;

  match reversed_filesystem.next() {
    None => None,
    Some((index, option)) => {
      match option {
        None => find_next_block(reversed_filesystem),
        Some(file_id) => {
          loop {
            match reversed_filesystem.next() {
              None => { return Option()}
            }
          }
        }
      }
    }
  }
}
fn find_block_length(reversed_filesystem: &mut Rev<Enumerate<Iter<'_, Option<i32>>>>, file_id: &Option<i32>) -> usize {
  let mut length = 0;

  match reversed_filesystem.next() {
    None => length,
    Some((index, option) =>
  }
}
  */
  */
  */

fn checksum(filesystem: &Vec<Option<i32>>) -> usize {
  filesystem.iter().enumerate().fold(0, |checksum, (index, item)| {
    match item {
      Some(file_id) => checksum + (index * (*file_id as usize)),
      None => checksum
    }
  })
}
