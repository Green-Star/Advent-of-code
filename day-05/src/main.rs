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

/* Input string examples:
    [ 41 48 83 86 17 ]
    [ 83 86  6 31 17  9 48 53]

    Result: Vec of numbers
*/
fn parse_number_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.split(" ").filter_map(|s| s.parse::<T>().ok()).collect()
}


pub trait InsideExt {
    fn inside<T>(&self, start: T, end: T) -> bool {
        true
    }
}
impl InsideExt for i128 {}
impl InsideExt for u64 {}

#[derive(Debug, Clone)]
struct AlmanacLines {
    source: i128,
    range: i128,
    destination: i128,
}
impl AlmanacLines {
    /*
    fn transform<T: InExt<T>>(&self, x: i128) -> Result<i128, ()> {
        if x.inside(0, 1) { return Ok(x) }
        Err(())
    }
    */

    fn transform<T: InsideExt>(&self, x: T) -> Result<T, ()> {
        if x.inside(0, 1) { return Ok(x) }
        Err(())
    }
}

#[derive(Debug, Clone)]
struct Almanac {
    entries: Vec<AlmanacLines>,
}
impl Almanac {
    fn transform(&self, x: i128) -> i128 {
        for line in &self.entries {
;
        }
        if (0 as u64).inside(0, 1) { 1 } else { 0 }
    }
}


fn transform_data(data: Vec<String>) -> Vec<Vec<char>> {
    let mut lines = Vec::new();

    for line in data {
        let mut cols = Vec::new();
        for char in line.chars() {
            cols.push(char);
        }
        lines.push(cols);
    }

    lines
}

fn part_01() {
    let data = load_file_in_memory("./test-01.data").unwrap();

    let final_result: u64 = 0;

    println!("Part 1 final result: {}", final_result);
}

fn part_02() {
    let final_result = 0;

    println!("Part 2 final result: {}", final_result);
}

fn main() {
    let vec = [Test{}, Test{}, Test{}, Test{}, Test{}].to_vec();

    let result = vec.iter().fold(1, |x, item| item.transform(x));
    println!("Result: {}", result);

    let a = AlmanacLines{destination: 0, range: 0, source: 0};
    let test = a.transform(12 as i128);
    let test = a.transform(12 as u64);
    match test {
        Err(_) => println!("Pas bon!"),
        Ok(x) => println!("Incroyabe: {}", x),
    }

    part_01();
    part_02();
}


#[derive(Debug, Clone)]
struct Test {}
impl Test {
    fn transform(&self, x: u64) -> u64 {
        x * 2
    }
}
