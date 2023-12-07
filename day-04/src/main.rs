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

fn parse_number_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.split(" ").filter_map(|s| s.parse::<T>().ok()).collect()
}


fn get_winning_number_match_occurences(winning_number: &u32, list_number: &[u32]) -> u32 {
    list_number.iter().map(|x| if x == winning_number { 1 } else { 0 }).sum()
}

fn get_card_value(match_occurences: u32) -> u64 {
    if match_occurences == 0 {
        0
    } else {
        2_u64.pow(match_occurences - 1)
    }
}

/*
fn get_match_occurences(game: Game) -> u32 {
    ((Vec<int>)game.winning_number_list).map(|win| get_winning_number_match_occurences(win, &game.number_list)).sum()
}
*/

fn part_01() {
    let data = load_file_in_memory("./test-01.data").unwrap();

    for s in data {
        println!("[{}]", s);
    }

    println!("Part 1 final result: {}", 0);
}

fn part_02() {
}

fn main() {
    part_01();
    part_02();
}
