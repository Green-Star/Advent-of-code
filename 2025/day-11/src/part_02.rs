use std::collections::HashMap;

pub fn resolve(s: &str) -> i32 {
    let mut rack = transform_data(s);
    rack.explore_path();
    let final_result = rack.devices.get("out").unwrap();
    *final_result
}

fn transform_data(data: &str) -> Rack {
    let mut device_map = HashMap::new();

    for l in data.lines() {
        let mut ss = l.split(":");

        let (device, outputs) = (ss.next().unwrap(), ss.last().unwrap());
        let mut next_devices = vec![];
        for o in outputs.split_whitespace() {
            next_devices.push(o.to_string());
        }

        device_map.insert(device.to_string(), next_devices);
    }

    let mut exploring = HashMap::new();
    // Always starting at 'you' with 1 path
    exploring.insert(String::from("you"), 1);

    Rack { device_map, devices: HashMap::new(), exploring, last_explored: HashMap::new() }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Rack {
    device_map: HashMap<String, Vec<String>>,
    devices: HashMap<String, i32>,

    exploring: HashMap<String, i32>,
    last_explored: HashMap<String, i32>,
}
impl Rack {
    fn explore_path(&mut self) {
        while !self.exploring.is_empty() {
            let mut next_exploring = HashMap::new();

            for (current_node, node_paths) in &self.exploring {
                self.last_explored.insert(current_node.clone(), *node_paths);
                for next in self.device_map.get(current_node).unwrap_or(&vec![]) {
                    self.devices.entry(next.clone())
                                .and_modify(|paths| *paths += node_paths - self.last_explored.get(next).unwrap_or(&0))
                                .or_insert(*node_paths);
                    if next != "out" {
                        next_exploring.entry(next.clone()).and_modify(|paths| *paths += *node_paths).or_insert(*node_paths);
                    }
                }
            }

            self.exploring = next_exploring;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let test_input = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

        assert_eq!(resolve(test_input), 5);
    }

    #[test]
    fn test_smaller_graph_01() {
        let test_input = "\
you: b d
d: out
c: d e
f: d
e: f
b: c
";

        assert_eq!(resolve(test_input), 3);
    }
    #[test]
    fn test_smaller_graph_02() {
        let test_input = "\
you: a c b
a: d
c: d
d: out
b: e
e: g
g: d
";

        assert_eq!(resolve(test_input), 3);
    }
    #[test]
    fn test_smallest_graph_02() {
        let test_input = "\
you: b a
a: out
b: a
";

        assert_eq!(resolve(test_input), 2);
    }
}
