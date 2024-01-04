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

fn transform_data(data: Vec<String>) -> Vec<Lava> {
    let mut result = Vec::new();

    let mut line = Lava { lines: Vec::new() };
    for s in data {
        if s.is_empty() {
            result.push(line);
            line = Lava { lines: Vec::new() };
        }
    }

    result
}

struct Lava {
    lines: Vec<Vec<char>>,
}

fn part_01(filepath: &str) {
    let data = load_file_in_memory(filepath).unwrap();

    println!("Part 1 final result: {}", 0);
}


fn part_02(filepath: &str) {
    let data = load_file_in_memory(filepath).unwrap();
}


fn main() {
    let now = Instant::now();
    part_01("./test.data");
    let elapsed: std::time::Duration = now.elapsed();
    println!("Part 1 found in {:?}s", elapsed.as_secs());
    let now = Instant::now();
    part_02("./test.data");
    let elapsed = now.elapsed();
    println!("Part 2 found in {:?}s", elapsed.as_secs());
}
