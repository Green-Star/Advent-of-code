use crate::core;
use crate::*;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn transform_data(data: Vec<String>) -> Vec<Vec<char>> {
    let mut transformed = Vec::new();
    for i in 0..data.len() {
        transformed.push(Vec::new());
        for c in data[i].chars() {
            transformed[i].push(c);
        }
    }
    expand_space(transformed)
}

fn expand_space(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let expanded = expand_columns(data);
    let expanded = expand_lines(expanded);
    expanded
}

fn expand_columns(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    for _ in 0..data.len() {
        result.push(Vec::new());
    }

    for j in 0..data[0].len() {
        let mut is_empty = true;

        for i in 0..data.len() {
            result[i].push(data[i][j]);
            match data[i][j] {
                '.' => continue,
                _ => is_empty = false,
            }
        }

        if is_empty {
            for i in 0..data.len() {
                result[i].push(data[i][j]);
            }
        }
    }

    result
}

fn expand_lines(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    for i in 0..data.len() {
        let mut is_empty = true;
        for j in 0..data[i].len() {
            match data[i][j] {
                '.' => continue,
                _ => { is_empty = false; break; },
            }
        }
        if is_empty {
            result.push(data[i].clone());
        }
        result.push(data[i].clone());
    }

    result
}

pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let space = transform_data(data);
    let galaxies = extract_galaxies(space);
    let pairs = get_pairs_of_galaxies(galaxies);

    let final_result = pairs.iter().fold(0, |sum, (x, y)| sum + x.get_distance(y));

    println!("Part 1 final result: {}", final_result);
}
