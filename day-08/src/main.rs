use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;

fn load_file_in_memory(filepath: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        data.push(line.unwrap());
    }

    Ok(data)
}


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

/*
struct Walker {
    grid: HashMap<String, Node>,

    steps: u32,
    current: Node,
}
impl Walker {
    fn new<'a>(grid: HashMap<String, Node>, start: &str) -> Walker {
        Walker { grid, steps: 0, current: grid.get(start).unwrap().clone() }
    }

    fn walk(&mut self, direction: &Direction) {
        self.steps += 1;
        match direction {
            Direction::Left => self.current = self.grid.get(&self.current.left_id).unwrap().clone(),
            Direction::Right => {},//self.current = grid.get(&self.current.right_id).unwrap(),
        }
    }
}
*/

/*
fn walk(walker: &mut Walker, grid: &HashMap<&str, Node>, direction: &Direction) {
    walker.steps += 1;
    walker.current = grid.get(walker.current.left_id).unwrap().clone();
}
*/

fn transform_data(data: Vec<String>) -> (Vec<Direction>, HashMap<String, Node>) {
    let mut directions = Vec::new();
    let mut grid = HashMap::new();

    for line in data {
        if line.is_empty() { continue; }

        let node_line = line.find("=");
        match line.find("=") {
            Some(_) => { let (node_id, node) = process_node_line(line); grid.insert(node_id, node); },
            None => directions.append(&mut process_direction_line(line)),
        };
    }

    (directions, grid)
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

fn part_01() {

}


fn main() {
    let now = Instant::now();
//    part_01::resolve();
    let elapsed = now.elapsed();
    println!("Part 1 found in {:?}s", elapsed.as_secs());
    let now = Instant::now();
//    part_02::resolve();
    let elapsed = now.elapsed();
    println!("Part 2 found in {:?}s", elapsed.as_secs());
}
