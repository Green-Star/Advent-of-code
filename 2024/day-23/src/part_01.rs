use std::collections::HashMap;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let map = transform_data(data);

  println!("{:?}", map);

  find_computer_sets(&map, "t");

  let final_result = 0;
  println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> NetworkMap {
  let mut map = HashMap::new();

  for line in data {
    let mut s = line.split("-");
    let (a, b) = (s.next().unwrap().to_string(), s.last().unwrap().to_string());

    map.entry(a.clone()).and_modify(|n: &mut Node| n.neighbour.push(b.clone())).or_insert(Node { node: a.clone(), neighbour: vec![b.clone()] });
    map.entry(b.clone()).and_modify(|n: &mut Node| n.neighbour.push(a.clone())).or_insert(Node { node: b.clone(), neighbour: vec![a.clone()] });
  }

  map.into_values().collect()
}

fn find_computer_sets(map: &NetworkMap, pattern: &str) {
  for n in map.iter().filter(|n| n.node.starts_with(pattern)) {
    println!("{:?}", n);
  }
}


type NetworkMap = Vec<Node>;
#[derive(Debug, Clone)]
struct Node {
  node: String,
  neighbour: Vec<String>,
}


#[derive(Debug, Clone)]
struct StateMachine {
  a: i32,
  b: i32,
  c: i32,

  instruction: usize,
  opcodes: Vec<i32>,

  outputs: Vec<i32>,
}
