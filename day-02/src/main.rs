use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_file_in_memory(filepath: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        data.push(line.unwrap());
    }

    Ok(data)
}

#[derive(PartialEq, Debug)]
struct Cubes {
    r: u32,
    g: u32,
    b: u32,
}
#[derive(Debug)]
struct Game {
    game_id: u32,
    _cubes_set: Vec<Cubes>,
    cubes_needed: Cubes,
}
impl Game {
    fn is_possible(&self, max_cubes_allowed: &Cubes) -> bool {
        self.cubes_needed.r <= max_cubes_allowed.r && self.cubes_needed.g <= max_cubes_allowed.g && self.cubes_needed.b <= max_cubes_allowed.b
    }
}

fn transform_data(data: Vec<String>) -> Vec<Game> {
    let mut result = Vec::new();

    for line in data {
        result.push(parse_game(&line));
    }

    result
}

fn parse_game(line: &str) -> Game {
    let mut parsed_line = line.split(':');
    let header = parsed_line.next().unwrap();
    let game_list = parsed_line.last().unwrap();

    let game_id = get_game_id(header);
    let _cubes_set = parse_cube_set(game_list);
    let cubes_needed = get_max_cubes_needed(&_cubes_set);

    Game { game_id, _cubes_set, cubes_needed }
}

fn get_game_id(game_header: &str) -> u32 {
    let mut result = 0;
    let parsed = game_header.split(' ');
    for part in parsed {
        match part {
            "Game" => {},
            id => { result = id.parse().unwrap(); },
        }
    }

    result
 }

fn parse_cube_set(s: &str) -> Vec<Cubes> {
    let mut result = Vec::new();

    let cube_sets_list = s.split(';');
    for cube_set in cube_sets_list {
        result.push(transform_into_cube(cube_set));
    }

    result
}

fn transform_into_cube(s: &str) -> Cubes {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    let cube_detail = s.split(',');
    for items in cube_detail {
        let mut parsed_detail = items.trim().split(' ');
        match (parsed_detail.next(), parsed_detail.next()) {
            (Some(number), Some("red")) => { red = number.parse().unwrap() },
            (Some(number), Some("green")) => { green = number.parse().unwrap() },
            (Some(number), Some("blue")) => { blue = number.parse().unwrap() },
            _ => {}
        }
    }

    Cubes { r: red, g: green, b: blue }
}

fn get_max_cubes_needed(cubes_set: &Vec<Cubes>) -> Cubes {
    let mut r_max = 0;
    let mut g_max = 0;
    let mut b_max = 0;

    for cube in cubes_set {
        r_max = max(r_max, cube.r);
        g_max = max(g_max, cube.g);
        b_max = max(b_max, cube.b);
    }

    Cubes { r: r_max, g: g_max, b: b_max }
}

fn main() {
    let max_cubes_allowed = Cubes { r: 12, g: 13, b: 14 };

    let data = load_file_in_memory("./input.data").unwrap();
    let game_list = transform_data(data);
    let possible_game_list = game_list.iter().filter(|game| game.is_possible(&max_cubes_allowed));
    let final_result: u32 = possible_game_list.map(|game| game.game_id).sum();

    println!("Part 1 final result: {}", final_result);
}
