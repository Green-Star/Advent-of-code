use std::collections::HashMap;

pub fn resolve(s: &str) -> i64 {
    let rack = transform_data(s);

    let subgraph = rack.explore_path(&String::from("svr"), &String::from("fft"));
    let &svr_to_fft = subgraph.get("fft").unwrap_or(&0);
    let subgraph = rack.explore_path(&String::from("fft"), &String::from("dac"));
    let &fft_to_dac = subgraph.get("dac").unwrap_or(&0);
    let subgraph = rack.explore_path(&String::from("dac"), &String::from("out"));
    let &dac_to_out = subgraph.get("out").unwrap_or(&0);


    let subgraph = rack.explore_path(&String::from("svr"), &String::from("dac"));
    let &srv_to_dac = subgraph.get("dac").unwrap_or(&0);
    let subgraph = rack.explore_path(&String::from("dac"), &String::from("fft"));
    let &dac_to_fft = subgraph.get("fft").unwrap_or(&0);
    let subgraph = rack.explore_path(&String::from("fft"), &String::from("out"));
    let &fft_to_out = subgraph.get("out").unwrap_or(&0);

    let final_result = (svr_to_fft * fft_to_dac * dac_to_out) + (srv_to_dac * dac_to_fft * fft_to_out);
    final_result
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

    Rack { device_map }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Rack {
    device_map: HashMap<String, Vec<String>>,
}
impl Rack {
    fn explore_path(&self, start: &String, end: &String) -> HashMap<String, i64> {
        let mut last_explored = HashMap::new();
        let mut explored_devices = HashMap::new();

        let mut exploring = HashMap::new();
        exploring.insert(start.clone(), 1);

        while !exploring.is_empty() {
            let mut next_exploring = HashMap::new();

            for (current_node, node_paths) in &exploring {
                last_explored.insert(current_node.clone(), *node_paths);
                for next in self.device_map.get(current_node).unwrap_or(&vec![]) {
                    explored_devices.entry(next.clone())
                                    .and_modify(|paths| *paths += node_paths - last_explored.get(next).unwrap_or(&0))
                                    .or_insert(*node_paths);
                    if next != end {
                        next_exploring.entry(next.clone()).and_modify(|paths| *paths += *node_paths).or_insert(*node_paths);
                    }
                }
            }

            exploring = next_exploring;
        }

        explored_devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_02() {
        let test_input = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

        assert_eq!(resolve(test_input), 2);
    }


    #[test]
    fn test_part_02_details() {
        let test_input = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

        let rack = transform_data(test_input);
        /* Test SRV -> FFT -> DAC -> OUT */
        let explored_devices = rack.explore_path(&String::from("svr"), &String::from("fft"));
        assert_eq!(*explored_devices.get("fft").unwrap_or(&0), 1);
        let explored_devices = rack.explore_path(&String::from("fft"), &String::from("dac"));
        assert_eq!(*explored_devices.get("dac").unwrap_or(&0), 1);
        let explored_devices = rack.explore_path(&String::from("dac"), &String::from("out"));
        assert_eq!(*explored_devices.get("out").unwrap_or(&0), 2);

        /* And test SRV -> DAC -> FFT -> OUT */
        let explored_devices = rack.explore_path(&String::from("svr"), &String::from("dac"));
        assert_eq!(*explored_devices.get("dac").unwrap_or(&0), 2);
        let explored_devices = rack.explore_path(&String::from("dac"), &String::from("fft"));
        assert_eq!(*explored_devices.get("fft").unwrap_or(&0), 0);
        let explored_devices = rack.explore_path(&String::from("fft"), &String::from("out"));
        assert_eq!(*explored_devices.get("out").unwrap_or(&0), 4);

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

        let rack = transform_data(test_input);
        let explored_devices = rack.explore_path(&String::from("you"), &String::from("out"));
        assert_eq!(*explored_devices.get("out").unwrap(), 3);
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

        let rack = transform_data(test_input);
        let explored_devices = rack.explore_path(&String::from("you"), &String::from("out"));
        assert_eq!(*explored_devices.get("out").unwrap(), 3);
    }
    #[test]
    fn test_smallest_graph_02() {
        let test_input = "\
you: b a
a: out
b: a
";

        let rack = transform_data(test_input);
        let explored_devices = rack.explore_path(&String::from("you"), &String::from("out"));
        assert_eq!(*explored_devices.get("out").unwrap(), 2);
    }
}
