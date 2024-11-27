use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::TryFromIntError;
use std::rc::Rc;
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

fn transform_data(data: Vec<String>) -> Vec<Vec<Node>> {
    let mut map = Vec::new();
    let mut line = 0;

    for s in data {
        let mut node_line = Vec::new();
        let mut column = 0;
        for c in s.chars() {
            node_line.push(Node::new(Tile::from(&c), line, column));
            column += 1;
        }
        map.push(node_line);
        line += 1;
    }

    map
}


#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,

    StartingNode,
    None,
}
impl Tile {
    fn get_paths(tile: &Tile) -> ((isize, isize), (isize, isize)) {
        match tile {
            Self::NorthSouth => ((-1, 0), (1, 0)),
            Self::EastWest => ((0, 1), (0, -1)),
            Self::NorthEast => ((-1, 0), (0, 1)),
            Self::NorthWest => ((-1, 0), (0, -1)),
            Self::SouthWest | Self::StartingNode => ((1, 0), (0, -1)),  /* Starting node is a south west node */
            Self::SouthEast /* | Self::StartingNode */ => ((1, 0), (0, 1)),   /* In exmaples, starting node is a south east node */
            _ => panic!("Tile: Can't move from tile {:?}", tile),
        }
    }

    fn from(c: &char) -> Tile {
        match c {
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            'S' => Tile::StartingNode,
            _ => Tile::None,
        }
    }
}


#[derive(Debug, Copy, Clone)]
struct Node {
    steps_away: Option<u32>,
    tile: Tile,

    line: usize,
    column: usize,
}
impl Node {
    fn new(tile: Tile, line: usize, column: usize) -> Node {
        Node { steps_away: None, tile, line, column }
    }
}

fn get_next_nodes<'a>(map: &'a mut Vec<Vec<Node>>, current: &Node) -> (Option<&'a mut Node>, Option<&'a mut Node>) {
    let mut first_next_node = None;
    let mut second_next_node = None;
    let (first_direction, second_direction) = Tile::get_paths(&(current.tile));

    let new_indexes = get_next_indexes(map, current, first_direction);
    match new_indexes {
        Some((new_line_index, new_column_index)) => {
            first_next_node = Some(&mut map[current.line + new_line_index][current.column + new_column_index])
        },
        None => {
            first_next_node = None
        },
    }

/*
    let new_indexes = get_next_indexes(map, current, second_direction);
    match new_indexes {
        Some((new_line_index, new_column_index)) => {
            second_next_node = Some(&mut map[current.line + new_line_index][current.column + new_column_index])
        },
        None => {
            second_next_node = None
        },
    }
    */

    (first_next_node, second_next_node)
}

fn get_next_indexes(map: &Vec<Vec<Node>>, current: &Node, offset: (isize, isize)) -> Option<(usize, usize)> {
    println!("({},{})+({},{})", current.line, current.column, offset.0, offset.1);
    if offset.0 == -1 && current.line == 0 { None }
    else if offset.0 == 1 && current.line == map.len() { None }
    else if offset.1 == -1 && current.column == 0 { None }
    else if offset.1 == 1 && current.column == map[current.line].len() { None }
    else { Some((current.line.checked_add_signed(offset.0).unwrap(), current.column.checked_add_signed(offset.1).unwrap())) }
}

fn init_map(map: &mut Vec<Vec<Node>>) -> Result<&Node, ()>{
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y].tile == Tile::StartingNode {
                map[x][y].steps_away = Some(0);
                return Ok(&map[x][y]);
            }
        }
    }
    Err(())
}

/*
trait Ext: ExactSizeIterator {
    fn get_next_nodes(&self, current: &Node) -> (Option<Node>, Option<Node>) {
        let mut first_next_node = None;
        let mut second_next_node = None;
        let (first_direction, second_direction) = Tile::get_paths(&(current.tile));

        if first_direction.0 == -1 && current.line == 0 {}
        else if first_direction.0 == 1 && current.line == self.len() {}
        else if first_direction.1 == -1 && current.column == 0 {}
        else if first_direction.1 == 1 && current.column == self[current.line].len() {}

        (first_next_node, second_next_node)
    }
}
*/
/*
impl Ext for Vec<Vec<Node>> {

}
*/
struct Map {
    data: Vec<Vec<Node>>,
    starting_node: Node,
}
impl Map {
    fn new(data: Vec<Vec<Node>>) -> Map {
        let mut starting_node = None;
        let mut owned_data = data;

        for x in 0..owned_data.len() {
            for y in 0..owned_data[x].len() {
                if owned_data[x][y].tile == Tile::StartingNode {
                    owned_data[x][y].steps_away = Some(0);
                    starting_node = Some(owned_data[x][y]);
                }
            }
        }

        Map { data: owned_data, starting_node: starting_node.unwrap() }
    }
}
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                match self.data[i][j].steps_away {
                    Some(step) => { write!(f, "[{}]", step).unwrap() },
                    None => { write!(f, "[.]").unwrap() },
                }
            }
            writeln!(f, "").unwrap();
        }
        Ok(())
    }
}

struct WalkerOld<'a> {
    map: &'a mut Vec<Vec<Node>>,

    steps: u32,
    current: &'a Node,

    done: bool,
}
/*
impl Walker<'_> {
    fn new<'a>(map: &'a mut Vec<Vec<Node>>, start: &'a Node) -> Walker<'a> {
        Walker { map, steps: 0, current: start, done: false }
    }

    fn start<'a>(&'a mut self, starting_node: &'a mut Node) {
        self.steps += 1;
        self.current = starting_node;
        self.map[self.current.line][self.current.column].steps_away = Some(self.steps);
    }

    fn is_finished(&self) -> bool {
        self.done == true
    }

    fn walk(&mut self) {
        let next_steps = get_next_nodes(self.map, self.current);

        self.steps += 1;

        let mut has_moved = false;
        match next_steps.0 {
            None => {},
            Some(node) => {
                match node.steps_away {
                    None => {
                        self.move_to_node(node);
                        has_moved = true;
                    },
                    Some(steps_away) => {
                        if steps_away > self.steps {
                            self.move_to_node(node);
                            has_moved = true;
                        }
                    }
                }
            }
        }

        match next_steps.1 {
            None => {},
            Some(node) => {
                match node.steps_away {
                    None => {
                        self.move_to_node(node);
                        has_moved = true;
                    },
                    Some(steps_away) => {
                        if steps_away > self.steps {
                            self.move_to_node(node);
                            has_moved = true;
                        }
                    }
                }
            }
        }

        if has_moved == false { self.done = true; }
    }

    fn move_to_node(&mut self, next: &mut Node) {
        next.steps_away = Some(self.steps);
        self.current = next;
    }
}
*/

struct Walker {
    steps: u32,

    line: usize,
    column: usize,

    done: bool,
}

pub fn part_01(input_data_path: &str) {
    let data = load_file_in_memory(input_data_path).unwrap();
    let map = transform_data(data);
    let mut map = Map::new(map);

    let mut texas = Walker { steps: 0, line: map.starting_node.line, column: map.starting_node.column, done: false };
    let mut ranger = Walker { steps: 0, line: map.starting_node.line, column: map.starting_node.column, done: false };

    let (first_direction, second_direction) = Tile::get_paths(&(map.starting_node.tile));

    println!("Starting node: ({},{})", map.starting_node.line, map.starting_node.column);

    let new_indexes = get_next_indexes(&map.data, &map.starting_node, first_direction);
    match new_indexes {
        Some((new_line_index, new_column_index)) => {
            texas.steps = 1;
            texas.line = new_line_index;
            texas.column = new_column_index;
            map.data[texas.line][texas.column].steps_away = Some(texas.steps);
        },
        None => {
            texas.done = true;
        },
    }

    let new_indexes = get_next_indexes(&map.data, &map.starting_node, second_direction);
    match new_indexes {
        Some((new_line_index, new_column_index)) => {
            ranger.steps = 1;
            ranger.line = new_line_index;
            ranger.column = new_column_index;
            map.data[ranger.line][ranger.column].steps_away = Some(ranger.steps);
        },
        None => {
            ranger.done = true;
        },
    }

    let mut walkers = vec![texas, ranger];
    loop {
        walkers.iter_mut().for_each(|walker| {
            println!("Start walker");
            let mut has_moved = false;
            let current_line = walker.line;
            let current_column = walker.column;

            let (first_direction, second_direction) = Tile::get_paths(&(map.data[current_line][current_column].tile));

            walker.steps += 1;

            let new_indexes = get_next_indexes(&map.data, &(map.data[current_line][current_column]), first_direction);
            match new_indexes {
                None => {},
                Some((new_line_index, new_column_index)) => {
                    let next_node = &(map.data[new_line_index][new_column_index]);
                    match next_node.steps_away {
                        None => {
                            println!("Going to ({},{}) -> ({},{})", walker.line, walker.column, new_line_index, new_column_index);
                            walker.line = new_line_index;
                            walker.column = new_column_index;
                            map.data[walker.line][walker.column].steps_away = Some(walker.steps);
                            has_moved = true;
                        },
                        Some(step) => {
                            if step > walker.steps {
                                println!("Going to ({},{}) -> ({},{})", walker.line, walker.column, new_line_index, new_column_index);
                                walker.line = new_line_index;
                                walker.column = new_column_index;
                                map.data[walker.line][walker.column].steps_away = Some(walker.steps);
                                has_moved = true;
                            }
                        }
                    }
                },
            }

            let new_indexes = get_next_indexes(&map.data, &(map.data[current_line][current_column]), second_direction);
            match new_indexes {
                None => {},
                Some((new_line_index, new_column_index)) => {
                    let next_node = &(map.data[new_line_index][new_column_index]);
                    match next_node.steps_away {
                        None => {
                            println!("Going to ({},{}) -> ({},{})", walker.line, walker.column, new_line_index, new_column_index);
                            walker.line = new_line_index;
                            walker.column = new_column_index;
                            map.data[walker.line][walker.column].steps_away = Some(walker.steps);
                            has_moved = true;
                        },
                        Some(step) => {
                            if step > walker.steps {
                                println!("Going to ({},{}) -> ({},{})", walker.line, walker.column, new_line_index, new_column_index);
                                walker.line = new_line_index;
                                walker.column = new_column_index;
                                map.data[walker.line][walker.column].steps_away = Some(walker.steps);
                                has_moved = true;
                            }
                        }
                    }
                },
            }

            if has_moved == false { walker.done = true }
            println!("End walker");
        });

        println!("-- End step --");

        let finish = walkers.iter().map(|w| w.done).reduce(|finished, done| finished && done ).unwrap();
        if finish { break }
    }


    println!("{map}");

    let mut max = None;
    for i in 0..map.data.len() {
        for j in 0..map.data[i].len() {
            match map.data[i][j].steps_away {
                None => {},
                Some(step) => {
                    match max {
                        None => max = Some(step),
                        Some(max_steps) => if step > max_steps { max = Some(step) }
                    }
                }
            }
        }
    }
    let final_result = max.unwrap();

    println!("Part 1 final result: {}", final_result);
}

fn main() {
    let now = Instant::now();
    /*
    part_01("./test-01.data");
    part_01("./test-02.data");
    */
    part_01("./input.data");
    let elapsed = now.elapsed();
    println!("Part 1 found in {:?}s", elapsed.as_secs());
    let now = Instant::now();
    //part_02::resolve();
    let elapsed = now.elapsed();
    println!("Part 2 found in {:?}s", elapsed.as_secs());
}
