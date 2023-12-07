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
    cubes_set: Vec<Cubes>,
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
    let cubes_set = parse_cube_set(game_list);
    let cubes_needed = get_max_cubes_needed(&cubes_set);

    Game { game_id, cubes_set, cubes_needed }
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
/*
fn get_final_result(game_list: &Iterator<Game>) -> i64 {
    game_list.map(|game| game.game_id).sum()
}
*/

fn main() {
    let max_cubes_allowed = Cubes { r: 12, g: 13, b: 14 };

    let data = load_file_in_memory("./test.data").unwrap();
    let game_list = transform_data(data);
    let possible_game_list = game_list.iter().filter(|game| game.is_possible(&max_cubes_allowed));
    let final_result: u32 = possible_game_list.map(|game| game.game_id).sum();

/*
    let origin = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    parse_game(origin);

    println!("Game id: {}", get_game_id("Game 1"));
    println!("Game id: {}", get_game_id("Game 2"));
    println!("Game id: {}", get_game_id("Game 10"));
    println!("Game id: {}", get_game_id("Game 75"));
    println!("Game id: {}", get_game_id("Game 100"));

    let s1 = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let s2 = "3 blue, 4 red";
    let s3 = " 1 red, 2 green, 6 blue";
    let s4 = " 2 green";

    let cube_set = parse_cube_set(s1);
    assert_eq!(cube_set, Vec::from([
        Cubes{r:4,g:0,b:3},
        Cubes{r:1,g:2,b:6},
        Cubes{r:0,g:2,b:0},
    ]));
    dbg!("[{}]", cube_set);

    let cube = transform_into_cube(s2);
    assert_eq!(cube, Cubes{r:4, g:0, b:3});
    dbg!("[{}]", cube);
    let cube = transform_into_cube(s3);
    assert_eq!(cube, Cubes{r:1, g:2, b:6});
    dbg!("[{}]", cube);
    let cube = transform_into_cube(s4);
    assert_eq!(cube, Cubes{r:0, g:2, b:0});
    dbg!("[{}]", cube);
*/

    println!("Part 1 final result: {}", final_result);
}
