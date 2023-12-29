use std::fs::File;
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

fn extract_galaxies(space: Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut result = Vec::new();

    for line in 0..space.len() {
        for column in 0..space[line].len() {
            match space[line][column] {
                '.' => {},
                _ => result.push(Galaxy { line, column }),
            }
        }
    }

    result
}

fn get_pairs_of_galaxies(galaxies: Vec<Galaxy>) -> Vec<(Galaxy, Galaxy)> {
    let mut result = Vec::new();
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            result.push((galaxies[i], galaxies[j]))
        }
    }
    result
}

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    line: usize,
    column: usize,
}
impl Galaxy {
    fn get_distance(&self, other: &Galaxy) -> usize {
        self.line.abs_diff(other.line) + self.column.abs_diff(other.column)
    }
}

pub fn part_01(input_data_path: &str) {
    let data = load_file_in_memory(input_data_path).unwrap();
    let space = transform_data(data);
    let galaxies = extract_galaxies(space);
    let pairs = get_pairs_of_galaxies(galaxies);

    let final_result = pairs.iter().fold(0, |sum, (x, y)| sum + x.get_distance(y));

    println!("Part 1 final result: {}", final_result);
}

fn main() {
    let now = Instant::now();
    part_01("./input.data");
    let elapsed: std::time::Duration = now.elapsed();
    println!("Part 1 found in {:?}s", elapsed.as_secs());
    let now = Instant::now();
    //part_02::resolve();
    let elapsed = now.elapsed();
    println!("Part 2 found in {:?}s", elapsed.as_secs());
}
