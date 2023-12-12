use std::cmp::min;
use std::fs::File;
use std::i128;
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

/* Input string examples:
    [ 41 48 83 86 17 ]
    [ 83 86  6 31 17  9 48 53]

    Result: Vec of numbers
*/
fn parse_number_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.split(" ").filter_map(|s| s.parse::<T>().ok()).collect()
}

#[derive(Debug)]
struct Race {
    length: u64,
    record: u64,
    possible_wins: u64,
}
impl Race {
    fn from(length: &u64, record: &u64) -> Self {
        Self { length: *length, record: *record, possible_wins: Race::get_possible_wins(*length, *record) }
    }

    fn get_possible_wins(length: u64, record: u64) -> u64 {
        let mut possible_wins = 0;
        println!("Length: {length}, Record: {record}");
        for i in 0 ..= length {
            if Race::get_distance(i, length) > record {
                possible_wins += 1;
            }
        }
        possible_wins
    }

    fn get_distance(holding_time: u64, length: u64) -> u64 {
        holding_time * (length - holding_time)
    }
}

fn parse_line(line: &str) -> Vec<u64> {
    let parsed_line = line.split(":");
    parse_number_list(parsed_line.last().unwrap())
}

fn transform_data(data: Vec<String>) -> Vec<Race> {
    let time_line = parse_line(&data[0]);
    let record_line = parse_line(&data[1]);
    let mut time_list = time_line.iter();
    let mut record_list = record_line.iter();
    let mut races = Vec::new();

    loop {
        match (time_list.next(), record_list.next()) {
            (Some(length), Some(record)) => races.push(Race::from(length, record)),
            (_, _) => break
        }
    }

    races
}

fn part_01() {
    let data = load_file_in_memory("./test-01.data").unwrap();
    let races = transform_data(data);
    let final_result = races.iter().fold(1, |wins_number, race| wins_number * race.possible_wins);

    println!("Part 1 final result: {}", final_result);
}

fn part_02() {

}


fn main() {
    let now = Instant::now();
    part_01();
    let elapsed = now.elapsed();
    println!("Part 1 found in {:?}s", elapsed.as_secs());
    let now = Instant::now();
    part_02();
    let elapsed = now.elapsed();
    println!("Optimized part 2 found in {:?}s", elapsed.as_secs());
}
