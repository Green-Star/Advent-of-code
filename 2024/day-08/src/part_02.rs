pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let _ = transform_data(data);

  let final_result = 0;

  println!("Part 2 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> Vec<Vec<char>> {
  let mut result = Vec::new();

  for line in data {
    let mut chars = Vec::new();
    for c in line.chars() {
      chars.push(c);
    }
    result.push(chars);
  }

  result
}
