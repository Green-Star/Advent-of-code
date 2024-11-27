use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn load_file_in_memory(filepath: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        data.push(line.unwrap());
    }

    Ok(data)
}


#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
impl Direction {
    fn from(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction"),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    left_id: String,
    right_id: String,
}
impl Node {
    fn from(left_id: String, right_id: String) -> Node {
        Node { left_id, right_id }
    }
}

mod part_01 {
    use std::collections::HashMap;
    use crate::*;

    struct Walker<'a> {
        grid: &'a HashMap<String, Node>,
        end: &'a str,

        steps: u32,
        current: &'a str,
    }
    impl Walker<'_> {
        fn new<'a>(grid: &'a HashMap<String, Node>, start: &'a str, end: &'a str) -> Walker<'a> {
            Walker { grid, steps: 0, current: start, end }
        }

        fn walk(&mut self, direction: &Direction) {
            self.steps += 1;
            match direction {
                Direction::Left => self.current = &self.grid.get(self.current).unwrap().left_id,
                Direction::Right => self.current = &self.grid.get(self.current).unwrap().right_id,
            }
        }

        fn is_arrived(&self) -> bool {
            if self.current == self.end { return true } else { return false }
        }
    }


    fn transform_data(data: Vec<String>) -> (Vec<Direction>, HashMap<String, Node>) {
        let mut directions = Vec::new();
        let mut grid = HashMap::new();

        for line in data {
            if line.is_empty() { continue; }

            match line.find("=") {
                None => directions.append(&mut process_direction_line(line)),
                Some(_) => {
                    let (node_id, node) = process_node_line(line);
                    grid.insert(node_id, node);
                },
            };
        }

        (directions, grid)
    }

    pub fn resolve() {
        let data = load_file_in_memory("./input.data").unwrap();
        let (directions, map) = transform_data(data);

        let mut texas_ranger: Walker = Walker::new(&map, "AAA", "ZZZ");
        'directions: loop {
            for next_step in &directions {
                if texas_ranger.is_arrived() { break 'directions }

    println!("Step [{}] : Being at [{}] -> ([{}], [{}]), going {:?}", texas_ranger.steps, texas_ranger.current, &(map.get(texas_ranger.current).unwrap().left_id), &(map.get(texas_ranger.current).unwrap().right_id), next_step);

                texas_ranger.walk(next_step);
            }
        }


        let final_result = texas_ranger.steps;

        println!("Part 1 final result: {}", final_result);
    }

}

fn process_node_line(line: String) -> (String, Node) {
    let mut splitted_line = line.split(" = ");
    let id = splitted_line.next().unwrap();
    let nodes = splitted_line.next().unwrap();
    let nodes = nodes.replace("(", "").replace(")", "");
    let mut nodes = nodes.split(", ");
    let (left_id, right_id) = (nodes.next().unwrap(), nodes.next().unwrap());

    (id.to_string(), Node::from(left_id.to_string(), right_id.to_string()))
}
fn process_direction_line(line: String) -> Vec<Direction> {
    let mut directions = Vec::new();

    for c in line.chars() {
        directions.push(Direction::from(c));
    }

    directions
}

mod part_02 {
    use std::collections::HashMap;
    use crate::*;

    struct Walker<'a> {
        grid: &'a HashMap<String, Node>,

        steps: u32,
        current: &'a str,

        save_start: String,
    }
    impl Walker<'_> {
        fn new<'a>(grid: &'a HashMap<String, Node>, start: &'a str) -> Walker<'a> {
            Walker { grid, steps: 0, current: start, save_start: start.to_string() }
        }

        fn walk(&mut self, direction: &Direction) {
            self.steps += 1;
            match direction {
                Direction::Left => self.current = &self.grid.get(self.current).unwrap().left_id,
                Direction::Right => self.current = &self.grid.get(self.current).unwrap().right_id,
            }
        }

        fn is_arrived(&self) -> bool {
            if self.current.ends_with("Z") { return true } else { return false }
        }
    }

    fn transform_data(data: Vec<String>) -> (Vec<Direction>, Vec<String>, HashMap<String, Node>) {
        let mut directions = Vec::new();
        let mut grid = HashMap::new();
        let mut start_nodes = Vec::new();

        for line in data {
            if line.is_empty() { continue; }

            match line.find("=") {
                None => directions.append(&mut process_direction_line(line)),
                Some(_) => {
                    let (node_id, node) = process_node_line(line);
                    if node_id.ends_with("A") { start_nodes.push(node_id.clone()) };
                    grid.insert(node_id, node);
                },
            };
        }

        (directions, start_nodes, grid)
    }


    pub fn resolve() {
        let data = load_file_in_memory("./input.data").unwrap();
        let (directions, starting_nodes, map) = transform_data(data);

        let mut walkers: Vec<Walker> = starting_nodes.iter().map(|node| Walker::new(&map, node)).collect();
        println!("Using {} walkers...", walkers.len());

        'directions: loop {
            for next_step in &directions {
                for w in &walkers {
                    if w.is_arrived() {
                        println!("Walker started at ({}) arrived at [{}] after [{}] steps", w.save_start, w.current, w.steps);
                    }
                }

                let finished = walkers.iter().map(|walker| walker.is_arrived()).reduce(|all_finished, finished| all_finished == true && finished == true).unwrap();
                if finished { break 'directions }

                walkers.iter_mut().for_each(|walker| walker.walk(next_step));
            }
        }

        /* All walkers have walked the same amout of steps, so let's just use the first as the result */
        let final_result = walkers[0].steps;

        println!("Part 2 final result: {}", final_result);
    }
}


fn main() {
    let now = Instant::now();
    part_01::resolve();
    let elapsed = now.elapsed();
    println!("Part 1 found in {:?}s", elapsed.as_secs());
    let now = Instant::now();
    part_02::resolve();
    let elapsed = now.elapsed();
    println!("Part 2 found in {:?}s", elapsed.as_secs());
}
