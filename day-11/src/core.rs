use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_file_in_memory(filepath: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        data.push(line.unwrap());
    }

    Ok(data)
}

pub fn extract_galaxies(space: Vec<Vec<char>>) -> Vec<Galaxy> {
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

pub fn get_pairs_of_galaxies(galaxies: Vec<Galaxy>) -> Vec<(Galaxy, Galaxy)> {
    let mut result = Vec::new();
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            result.push((galaxies[i], galaxies[j]))
        }
    }
    result
}

#[derive(Debug, Clone, Copy)]
pub struct Galaxy {
    line: usize,
    column: usize,
}
impl Galaxy {
    pub fn get_distance(&self, other: &Galaxy) -> usize {
        self.line.abs_diff(other.line) + self.column.abs_diff(other.column)
    }
}
