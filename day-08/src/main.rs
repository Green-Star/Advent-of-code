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
    let data = load_file_in_memory("./input.data").unwrap();
    let (directions, map) = transform_data(data);
/*
    let starting_id = "AAA".to_string();
    let end_id = "ZZZ";

    let mut walker = (&starting_id, map.get(&starting_id).unwrap());
    let mut steps = 0;

    'directions: loop {
        for next_step in &directions {
            if walker.0 == end_id { break 'directions }

println!("Step [{}] : Being at [{}] -> ([{}], [{}]), going {:?}", steps, &walker.0, &walker.1.left_id, &walker.1.right_id, next_step);

            steps += 1;
            match next_step {
                Direction::Left => walker = (&(map.get(walker.0).unwrap().left_id), map.get(&map.get(walker.0).unwrap().left_id).unwrap()),
                Direction::Right => walker = (&(map.get(walker.0).unwrap().right_id), map.get(&map.get(walker.0).unwrap().right_id).unwrap()),
            }
        }
    }
*/
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


fn main() {
    let now = Instant::now();
    part_01();
    let elapsed = now.elapsed();
    println!("Part 1 found in {:?}s", elapsed.as_secs());
    let now = Instant::now();
//    part_02::resolve();
    let elapsed = now.elapsed();
    println!("Part 2 found in {:?}s", elapsed.as_secs());
}
