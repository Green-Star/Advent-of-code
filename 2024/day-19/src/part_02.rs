/*
use std::collections::{HashMap, HashSet};

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let mut map = transform_data(data);

  println!("{:?}", map);

  map.get_all_designs();
  //  let possible_designs = try_building_all_designs(&designs, &patterns);
  println!("{:?}", map);

  let final_result: i64 = map.get_final_value();
  println!("Part 2 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> DesignMap {
  let patterns: Vec<String> = data[0].split(",").map(|s| s.trim().to_string()).collect();
  let designs: Vec<String> = data.iter().skip(2).map(|s| s.to_string()).collect();
  let length_max = designs.iter().map(|d| d.len()).max().unwrap();
  let hashmap = HashMap::from_iter(patterns.iter().map(|p| (p.to_string(), 1)));
  let hashset = HashSet::from_iter(patterns.iter().map(|p| p.to_string()));

  /*
  let mut hashset = HashSet::new();
  hashset.insert("g".to_string());
  hashset.insert("gb".to_string());
  */

  DesignMap {
    designs,
    patterns,

    design_length_max: length_max,
    design_to_process: hashset,

    semi_finished_design: hashmap,
    finished_design: HashMap::new()
  }
}

type Pattern = String;
type Design = String;

#[derive(Debug, Clone)]
struct DesignMap {
  designs: Vec<Design>,
  patterns: Vec<Pattern>,

  design_length_max: usize,
  design_to_process: HashSet<Design>,

  semi_finished_design: HashMap<Design, i64>,
  finished_design: HashMap<Design, i64>,
}
impl DesignMap {
  fn get_all_designs(&mut self) {
    while self.design_to_process.is_empty() == false {
      let design_to_process = self.design_to_process.clone();

      self.design_to_process = HashSet::new();
      for d in design_to_process {
        self.build_next_design(d);
      }
    }
  }

  fn build_next_design(&mut self, design: Design) {
    if self.designs.contains(&design) { self.finished_design.entry(design).and_modify(|e| *e *= 2).or_insert(1); return }
    if design.len() >= self.design_length_max { return }

    for p in &self.patterns {
      let next_design = design.clone() + p;

      if self.designs.iter().any(|design| design.starts_with(&next_design)) == false { continue; }

      self.design_to_process.insert(next_design.clone());
      self.semi_finished_design.entry(next_design).and_modify(|e| *e *= 2).or_insert(1);
    }


  }

  fn get_final_value(&self) -> i64 {
    self.finished_design.iter().map(|(_, value)| value).sum()
  }
}

fn try_building_all_designs(designs: &Vec<Design>, patterns: &Vec<Pattern>) -> Vec<i64> {
  designs.iter().enumerate().filter_map(|(i, d)| {println!("Checking design {i}-{d}"); return try_building_design(d, patterns);}).collect()
}

fn try_building_end(design: &Design, patterns: &Vec<Pattern>) -> Option<()> {
  if design.is_empty() { return Some(()) }

  let result: Vec<()> = patterns.iter().filter_map(|p| {
    if design.starts_with(p) {
      return try_building_end(&String::from_iter(design.chars().skip(p.len())), patterns)
    }
    None
  }).collect();

  match result.is_empty() {
    true => None,
    false => Some(()),
  }
/*
  for p in patterns {
    if design.starts_with(p) {
      let is_possible = try_building_end_of_design(&String::from_iter(design.chars().skip(p.len())), patterns);
      if let Some(_) = is_possible { return is_possible }
    }
  }

  None
  */
}

fn try_building_design(design: &Design, patterns: &Vec<Pattern>) -> Option<i64> {
  if design.is_empty() { return Some(1) }

  let mut posssible_designs = 0;
  for p in patterns {
    if design.starts_with(p) {
      let is_possible = try_building_design(&String::from_iter(design.chars().skip(p.len())), patterns);
      if let Some(ok) = is_possible { posssible_designs += ok }
    }
  }

  if posssible_designs > 0 { Some(posssible_designs) }
  else { None }
}
*/

use std::collections::HashMap;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let (patterns, designs) = transform_data(data);

  println!("{:?}", patterns);
  println!("{:?}", designs);

  let possible_designs = try_building_all_designs(&designs, &patterns);

  let final_result: i64 = possible_designs.iter().sum();
  println!("Part 2 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> (Vec<Pattern>, Vec<Design>) {
  let patterns: Vec<String> = data[0].split(",").map(|s| s.trim().to_string()).collect();
  let designs: Vec<String> = data.iter().skip(2).map(|s| s.to_string()).collect();

  (patterns, vec![designs[0].clone()])
}

type Pattern = String;
type Design = String;

fn try_building_all_designs(designs: &Vec<Design>, patterns: &Vec<Pattern>) -> Vec<i64> {
  designs.iter().map(|d| try_building_design(d, patterns, &mut HashMap::new())).collect()
}

fn try_building_design(design: &Design, patterns: &Vec<Pattern>, map: &mut HashMap<Design, i64>) -> i64 {
  if let Some(value) = map.get(design) { return *value; }
  if design.is_empty() { return 1 ; }

  let mut possibilities = 0;
  for p in patterns {
    if p.len() > design.len() { continue; }
    if design.starts_with(p) == false { continue; }

    possibilities += try_building_design(&String::from_iter(design.chars().skip(p.len())), patterns, map);
  }

  map.entry(design.clone()).and_modify(|e| *e += possibilities).or_insert(possibilities);
  possibilities
  /*

  let mut ways = 0;

  for p in patterns {
    if design.is_empty() { return 1 }
    if design.starts_with(p) {
      ways += try_building_design(&String::from_iter(design.chars().skip(p.len())), patterns);
    }
  }

  ways
  */
}
