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

fn transform_data(data: Vec<String>) -> Vec<Lava> {
    let mut result = Vec::new();

    let mut line = Lava { lines: Vec::new() };
    let mut current_line = 0;
    for s in data {
        if s.is_empty() {
            result.push(line);
            line = Lava { lines: Vec::new() };
            current_line = 0;
            continue;
        }

        line.lines.push(Vec::new());
        for c in s.chars() {
            line.lines[current_line].push(c);
        }
        current_line += 1;
    }
    result.push(line);

    result
}

#[derive(Debug)]
struct Lava {
    lines: Vec<Vec<char>>,
}
impl Lava {
    fn find_horizontal_symmetry(&self) -> Option<usize> {
        for i in 1..self.lines.len() {
            let mut different = false;
            for j in 0..self.lines[i].len() {
                if self.lines[i-1][j] != self.lines[i][j] {
                    different = true;
                    break;
                }
            }
            if different == false {
                if self.check_symmetry_in_lines(i-1, i) { return Some(i); }
            }
        }
        None
    }

    fn check_symmetry_in_lines(&self, mut start: usize, mut end: usize) -> bool {
        loop {
            for j in 0..self.lines[start].len() {
                if self.lines[start][j] != self.lines[end][j] {
                    return false;
                }
            }

            if start == 0 || end == self.lines.len() - 1 {
                return true;
            }

            start -= 1;
            end += 1;
        }
    }

    fn find_vertical_symmetry(&self) -> Option<usize> {
        for j in 1..self.lines[0].len() {
            let mut different = false;
            for i in 0..self.lines.len() {
                if self.lines[i][j-1] != self.lines[i][j] {
                    different = true;
                    break;
                }
            }
            if different == false {
                if self.check_symmetry_in_columns(j-1, j) {
                    return Some(j);
                }
            }
        }
        None
    }

    fn check_symmetry_in_columns(&self, mut start: usize, mut end: usize) -> bool {
        loop {
            for i in 0..self.lines.len() {
                if self.lines[i][start] != self.lines[i][end] { return false }
            }

            if start == 0 || end == self.lines[0].len() - 1 {
                return true;
            }

            start -= 1;
            end += 1;
        }
    }

    fn get_value(&self) -> i64 {
        if let Some(value) = self.find_horizontal_symmetry() {
            i64::try_from(value).unwrap() * 100
        }
        else if let Some(value) = self.find_vertical_symmetry() {
            i64::try_from(value).unwrap()
        }
        else {
            0
        }
    }
}

fn part_01(filepath: &str) {
    let data = load_file_in_memory(filepath).unwrap();
    let lavas = transform_data(data);

    println!("[{}]", lavas.len());

    let final_result = lavas.iter().fold(0, |result, lava| result + lava.get_value());

    println!("Part 1 final result: {}", final_result);
}


fn part_02(filepath: &str) {
    let data = load_file_in_memory(filepath).unwrap();
}


fn main() {
    let now = Instant::now();
    part_01("./input.data");
    let elapsed: std::time::Duration = now.elapsed();
    println!("Part 1 found in {:?}s", elapsed.as_secs());
    let now = Instant::now();
    part_02("./test.data");
    let elapsed = now.elapsed();
    println!("Part 2 found in {:?}s", elapsed.as_secs());
}
