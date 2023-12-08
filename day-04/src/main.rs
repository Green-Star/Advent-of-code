use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Lines;

fn load_file_in_memory(filepath: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        data.push(line.unwrap());
    }

    Ok(data)
}

/* Input string examples:
    [ 41 48 83 86 17 ]
    [ 83 86  6 31 17  9 48 53]

    Result: Vec of numbers
*/
fn parse_number_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.split(" ").filter_map(|s| s.parse::<T>().ok()).collect()
}



#[derive(Debug)]
struct Card {
    game_id: u32,
    winning_number_list: Vec<u32>,
    number_list: Vec<u32>,
}

fn transform_data(data: Vec<String>) -> Vec<Card> {
    data.iter().map(|line| parse_card(line)).collect()
}

/* Input string examples:
    [Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53]
    [Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19]

    . Split string into :
        [[Card 1: 41 48 83 86 17], [ 83 86  6 31 17  9 48 53]]
        [[Card 2: 13 32 20 16 61 ], [ 61 30 68 82 17 32 24 19]]]
    . Parse substrings

    Result: Vec of Card
*/
fn parse_card(line: &String) -> Card {
    let mut parsed_line = line.split("|");
    let header = parsed_line.next().unwrap();

    let number_list = parse_number_list(parsed_line.last().unwrap());

    let (game_id, winning_number_list) = parse_card_header(header);

    Card { game_id, winning_number_list, number_list }
}

/* Input string examples:
    [Card 1: 41 48 83 86 17 ]
    [Card 2: 13 32 20 16 61 ]

    . Split string into :
        [[Card 1], [ 41 48 83 86 17 ]]
        [[Card 2], [ 13 32 20 16 61 ]]
    . Parse each substrings

    Result: (game_id (u32), Vec of u32 number)
*/
fn parse_card_header(header: &str) -> (u32, Vec<u32>) {
    let mut parsed_header = header.split(":");
    let card_header = parsed_header.next().unwrap();

    let winning_number_list = parse_number_list(parsed_header.last().unwrap());

    let mut game_id = 0;
    for s in card_header.split(" ") {
        match s {
            "Card" => {},
            id => { game_id = id.parse().unwrap() },
        }
    }

    (game_id, winning_number_list)
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


fn get_match_occurences(cards: &Card) -> u32 {
    cards.winning_number_list.iter().map(|win| get_winning_number_match_occurences(win, &cards.number_list)).sum()
}

fn part_01() {
    let data = load_file_in_memory("./test-01.data").unwrap();
    let card_list = transform_data(data);
    let occurences_list: Vec<u32> = card_list.iter().map(|card| get_match_occurences(card)).collect();
    let final_result: u64 = occurences_list.iter().map(|o| get_card_value(*o)).sum();

    println!("Part 1 final result: {}", final_result);
}

fn part_02() {
}

fn main() {
    part_01();
    part_02();
}
