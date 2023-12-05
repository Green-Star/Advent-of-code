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

fn get_first_and_last_numbers_in_line(line: &str) -> (i64, i64) {
    let first = 1;
    let last = 3;

    println!("[{}] -> ({}, {})", line, first, last);
    (first, last)
}

fn transform_data(lines: Vec<String>) -> Vec<(i64, i64)> {
    let mut transformed_data = Vec::new();

    for line in lines {
        transformed_data.push(get_first_and_last_numbers_in_line(&line));
    }

    transformed_data
}

fn get_number((first, last): (i64, i64)) -> i64 {
    let result = first * 10 + last;
    println!("({}, {}) -> {}", first, last, result);
    result
}

fn get_final_result(transformed_data: Vec<(i64, i64)>) -> i64 {
    let mut result = 0;

    for (first, last) in transformed_data {
        result += get_number((first, last));
    }

    result
}


fn main() {
    println!("Hello, world in proper Rust!");

    let data = load_file_in_memory("./test.data").unwrap();
    let numbers = transform_data(data);
    let result = get_final_result(numbers);

    println!("Final result: {}", result);
}
