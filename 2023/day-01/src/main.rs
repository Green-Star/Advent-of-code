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

#[derive(PartialEq, Clone)]
struct Number {
    char: String,
    value: u32,
}
#[derive(PartialEq)]
struct IndexedNumber {
    index: usize,
    number: Number,
}

fn get_first_and_last_numbers_in_line(line: &str) -> (u32, u32) {
    let needles = [
        Number{ char: String::from("1"), value: 1 },
        Number{ char: String::from("2"), value: 2 },
        Number{ char: String::from("3"), value: 3 },
        Number{ char: String::from("4"), value: 4 },
        Number{ char: String::from("5"), value: 5 },
        Number{ char: String::from("6"), value: 6 },
        Number{ char: String::from("7"), value: 7 },
        Number{ char: String::from("8"), value: 8 },
        Number{ char: String::from("9"), value: 9 },
        Number{ char: String::from("one"), value: 1 },
        Number{ char: String::from("two"), value: 2 },
        Number{ char: String::from("three"), value: 3 },
        Number{ char: String::from("four"), value: 4 },
        Number{ char: String::from("five"), value: 5 },
        Number{ char: String::from("six"), value: 6 },
        Number{ char: String::from("seven"), value: 7 },
        Number{ char: String::from("eight"), value: 8 },
        Number{ char: String::from("nine"), value: 9 },
    ];

    let mut first_number = None;
    let mut last_number = None;

    for number in needles {
        let number_first_occurence_index = line.find(&(number.char));
        let number_last_occurence_index = line.rfind(&(number.char));

        if number_first_occurence_index == None || number_last_occurence_index == None { continue; }

        first_number = match first_number {
            None => { Some(IndexedNumber { number: number.clone(), index: number_first_occurence_index.unwrap() }) },
            Some(x) if number_first_occurence_index.unwrap() < x.index => { Some(IndexedNumber { number: number.clone(), index: number_first_occurence_index.unwrap() }) },
            Some(x) => Some(x)
        };
        last_number = match last_number {
            None => { Some(IndexedNumber { number: number.clone(), index: number_last_occurence_index.unwrap() }) },
            Some(x) if number_last_occurence_index.unwrap() > x.index => { Some(IndexedNumber { number: number.clone(), index: number_last_occurence_index.unwrap() }) },
            Some(x) => Some(x)
        };
    }

    let first = first_number.unwrap().number.value;
    let last = last_number.unwrap().number.value;

    println!("[{}] -> ({}, {})", line, first, last);
    (first, last)
}

fn transform_data(lines: Vec<String>) -> Vec<(i64, i64)> {
    let mut transformed_data = Vec::new();

    for line in lines {
        let (x, y) = get_first_and_last_numbers_in_line(&line);
        transformed_data.push((i64::from(x), i64::from(y)));
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
    let data = load_file_in_memory("./input.data").unwrap();
    let numbers = transform_data(data);
    let result = get_final_result(numbers);

    println!("Part 2 final result: {}", result);
}
