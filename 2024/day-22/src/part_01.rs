use std::collections::{HashMap, HashSet};

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let map = transform_data(data);

  let final_result = 0;
  println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> NetworkMap {
  let mut map: HashMap<Node, Vec<Node>> = HashMap::new();

  for line in data {
    let mut s = line.split("-");
    let (a, b) = (s.next().unwrap().to_string(), s.last().unwrap().to_string());

    map.entry(a.clone()).and_modify(|neighbours| neighbours.push(b.clone())).or_insert(vec![ b.clone() ]);
    map.entry(b.clone()).and_modify(|neighbours| neighbours.push(a.clone())).or_insert(vec![ a.clone() ] );
  }

  map
}

fn find_computer_sets(map: &NetworkMap, pattern: &str) -> Vec<Vec<Node>> {
  let mut result = vec![];

  for n in map.keys().filter(|node| node.starts_with(pattern)) {
    if let Some(mut cycle) = find_cycle_from_vertice(map, n) {
      result.append(&mut cycle);
    }
  }

  result
}

fn find_cycle_from_vertice(map: &NetworkMap, start_vertice: &Node) -> Option<Vec<Vec<Node>>> {
  let mut result = vec![];

  for neighbour_node in map.get(start_vertice).unwrap() {
    if let Some(mut cycles) = find_cycle_of_3(map, start_vertice, neighbour_node) {
      result.append(&mut cycles);
    }
  }

  if result.is_empty() {
    None
  } else {
    Some(result)
  }
}

fn find_cycle_of_3(map: &NetworkMap, start: &Node, second: &Node) -> Option<Vec<Vec<Node>>> {
  let mut result = vec![];

  for third_node in map.get(second).unwrap().iter().filter(|node| *node != start) {
    if map.get(start).unwrap().contains(third_node) {
      result.push(vec![start.clone(), second.clone(), third_node.clone()]);
    }
  }

  if result.is_empty() {
    None
  } else {
    Some(result)
  }
}

fn drop_duplicate_cycles(cycles_list: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
  cycles_list.into_iter().map(|mut v| { v.sort(); v }).collect::<HashSet<_>>().into_iter().collect()
}

type Node = String;
type NetworkMap = HashMap<Node, Vec<Node>>;
