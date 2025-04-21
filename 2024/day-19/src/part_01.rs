pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let (patterns, designs) = transform_data(data);
  let possible_designs = try_building_all_designs(&designs, &patterns);

  let final_result = possible_designs.len();
  println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> (Vec<Pattern>, Vec<Design>) {
  let patterns: Vec<String> = data[0].split(",").map(|s| s.trim().to_string()).collect();
  let designs: Vec<String> = data.iter().skip(2).map(|s| s.to_string()).collect();

  (patterns, designs)
}

type Pattern = String;
type Design = String;

fn try_building_all_designs(designs: &Vec<Design>, patterns: &Vec<Pattern>) -> Vec<()> {
  designs.iter().filter_map(|d| try_building_design(d, patterns)).collect()
}

fn try_building_design(design: &Design, patterns: &Vec<Pattern>) -> Option<()> {
  if design.is_empty() { return Some(()) }

  for p in patterns {
    if design.starts_with(p) {
      let is_possible = try_building_design(&String::from_iter(design.chars().skip(p.len())), patterns);
      if let Some(_) = is_possible { return is_possible }
    }
  }

  None
}
