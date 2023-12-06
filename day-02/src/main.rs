use core::num;
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
struct Game {
    game_id: u32,
    cubes_set: Vec<Cubes>,
}

fn transform_data(data: Vec<String>) -> Vec<Game> {
    Vec::new()
}

fn parse_game(line: &str) -> Game {
    let mut parsed_line = line.split(':');
    let header = parsed_line.next().unwrap();
    let game_list = parsed_line.last().unwrap();

    let game_id = get_game_id(header);
    let cubes_set_list = parse_cube_set(game_list);

    Game { game_id: game_id, cubes_set: cubes_set_list }
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

fn get_final_result(numbers: i64) -> i64 {
    numbers
}

fn main() {
    println!("Hello, world for the 2nd day!");

    let data = load_file_in_memory("./test.data").unwrap();

    for s in data {
        println!("[{}]", s);
    }

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

    println!("Part 1 final result: {}", 0);
}
