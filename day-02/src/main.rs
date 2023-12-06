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

fn transform_data(data: Vec<String>) -> i64 {
    0
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

    //let numbers = transform_data(data);
    //let result = get_final_result(numbers);

    println!("Part 1 final result: {}", 0);
}
