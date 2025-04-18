use core::net;
use std::collections::HashMap;

type Node = String;
type NetworkMap = HashMap<Node, Vec<Node>>;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let map = transform_data(data);

  println!("{:?}", map);

  let mut connection_map = build_full_connections_map(&map);
  println!("Full network found: {:?}", connection_map);

  connection_map.sort_by(|a, b| b.len().cmp(&a.len()));
  let largest_network = connection_map.first().unwrap();
  println!("Larget network: {:?}", largest_network);

  let final_result = get_network_password(largest_network.to_vec());
  println!("Part 2 final result: {}", final_result);
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

fn build_full_connections_map(map: &NetworkMap) -> Vec<Vec<Node>> {
    let mut connections_map = vec![];

    for node in map.keys() {
        connections_map = build_network_with_node(connections_map, node, map);
    }

    connections_map
}

fn build_network_with_node(connections: Vec<Vec<Node>>, node: &Node, map: &NetworkMap) -> Vec<Vec<Node>> {
    let localhost = vec![ node.clone() ];
    let mut connections_map = vec![ localhost.clone() ];

    for network in connections {
        if node_can_be_added_to_network(map, node, &network) {
            let extended_network = vec![ network.clone(), localhost.clone() ].concat();
            connections_map.push(extended_network);
        }
        connections_map.push(network);
    }

    connections_map
}

fn node_can_be_added_to_network(map: &NetworkMap, node: &Node, network: &Vec<Node>) -> bool {
    network.iter().all(|vertice| map.get(vertice).unwrap().contains(node))
}

fn get_network_password(mut network: Vec<Node>) -> String {
    network.sort();
    network.join(",")
}
