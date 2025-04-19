use std::collections::HashMap;

type Node = String;
type NetworkMap = HashMap<Node, Vec<Node>>;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let map = transform_data(data);

  let final_result = 0;
  println!("Part 2 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> Vec<i64> {
    let mut result = vec![];

    for line in data {
      result.push(line.parse().unwrap());
    }

    result
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
