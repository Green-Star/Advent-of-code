use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

mod core;
mod part_01;
mod part_02;

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


fn main() {
    let now = Instant::now();
    part_01::resolve("./input.data");
    let elapsed: std::time::Duration = now.elapsed();
    println!("Part 1 found in {:?}s", elapsed.as_secs());
    let now = Instant::now();
    part_02::resolve("./test.data");
    let elapsed = now.elapsed();
    println!("Part 2 found in {:?}s", elapsed.as_secs());
}
