use std::collections::HashMap;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let (patterns, designs) = transform_data(data);
  let possible_designs = try_building_all_designs(&designs, &patterns);

  let final_result: i64 = possible_designs.iter().sum();
  println!("Part 2 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> (Vec<Pattern>, Vec<Design>) {
  let patterns: Vec<String> = data[0].split(",").map(|s| s.trim().to_string()).collect();
  let designs: Vec<String> = data.iter().skip(2).map(|s| s.to_string()).collect();

  (patterns, designs)
}

type Pattern = String;
type Design = String;

fn try_building_all_designs(designs: &Vec<Design>, patterns: &Vec<Pattern>) -> Vec<i64> {
  designs.iter().map(|d| try_building_design(d, patterns, &mut HashMap::new())).collect()
}

fn try_building_design(design: &Design, patterns: &Vec<Pattern>, map: &mut HashMap<Design, i64>) -> i64 {
  if let Some(value) = map.get(design) { return *value }
  if design.is_empty() { return 1 }

  let possibilities = patterns.iter()
                                  .filter(|p| p.len() <= design.len() && design.starts_with(*p))
                                  .map(|p| try_building_design(&String::from_iter(design.chars().skip(p.len())), patterns, map))
                                  .sum();

  map.entry(design.clone()).and_modify(|e| *e += possibilities).or_insert(possibilities);
  possibilities
}
