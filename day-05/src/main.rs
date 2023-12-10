use std::fs::File;
use std::{i128, result};
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



pub trait InsideExt
{
    fn inside<T>(&self, start: T, end: T) -> bool {
        true
    }
}
impl InsideExt for i128 {}

fn is_inside_right_open_interval<T: std::cmp::PartialOrd<T>>(x: &T, start: &T, end: &T) -> bool {
    if start <= x && x < end { true } else { false }
}

#[derive(Debug, Clone)]
struct AlmanacLines {
    source: i128,
    range: i128,
    destination: i128,
}
impl std::fmt::Display for AlmanacLines {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[source: {}, range: {}, destination: {}]", self.source, self.range, self.destination)
    }
}
impl AlmanacLines {
    /*
    fn transform<T: InExt<T>>(&self, x: i128) -> Result<i128, ()> {
        if x.inside(0, 1) { return Ok(x) }
        Err(())
    }
    */

    fn transform(&self, x: &i128) -> Result<i128, ()> {
        if is_inside_right_open_interval(x, &self.source, &(self.source + self.range)) { return Ok(self.destination + x - self.source) }
        Err(())
    }
}

#[derive(Debug, Clone)]
struct Almanac {
    entries: Vec<AlmanacLines>,
}
impl std::fmt::Display for Almanac {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Almanac(");
        for e in &self.entries { write!(f, "{}", e); };
        write!(f, ")")
    }
}
impl Almanac {
    fn transform(&self, x: i128) -> i128 {
        for line in &self.entries {
            match line.transform(&x) {
                Err(_) => {},
                Ok(result) => { return result }
            }
        }
        x
    }
}

fn extract_seeds(line: &str) -> Vec<i128> {
    let mut parsed_line = line.split(":");
    let (_, seeds) = (parsed_line.next(), parse_number_list(parsed_line.next().unwrap()));
    seeds
}

fn transform_data(data: Vec<String>) -> (Vec<i128>, Vec<Almanac>) {
    let mut seeds = [0, 1].to_vec();
    let mut book = Vec::new();
    let mut almanac_lines = Vec::new();

    for line in data {
        if line.is_empty() { continue; }
        /* if line.match: seeds: XXX, XXX */
        /*  Build seeds */
        if line.starts_with("seeds:") {
            seeds = extract_seeds(&line);
        }
        /* Build almanach */
        else if line.ends_with("map:") {
            /* line.match XXX map: */
            /*  Start new almanach entry: almanach.push(almanach_line) */
            if !(almanac_lines.is_empty()) {
                book.push(Almanac { entries: almanac_lines });
            }
            almanac_lines = Vec::new();
        }
        /*  Else : Add line entry  */
        else {
            almanac_lines.push(build_almanac_entry(&line));
        }
    }

    /* Save last almanac, if it exists */
    if !(almanac_lines.is_empty()) {
        book.push(Almanac { entries: almanac_lines });
    }

    (seeds, book)
}
fn build_almanac_entry(data: &str) -> AlmanacLines {
    let numbers_on_line = parse_number_list(data);
    let (destination, source, range) = (numbers_on_line[0], numbers_on_line[1], numbers_on_line[2]);
    AlmanacLines { source, range, destination }
}

fn part_01() {
    let data = load_file_in_memory("./test-01.data").unwrap();
    let (seeds, almanac) = transform_data(data);

    for s in seeds {
        print!("[{}], ", s);
    }
    for s in almanac {
        println!("{}", s);
    }

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

//    let test = 12.is_inside_right_open_interval(0, 1);
//    println!(":: {}", test);
/*
    let a = AlmanacLines{destination: 0, range: 0, source: 0};
    let test = a.transform(12 as i128);
    match test {
        Err(_) => println!("Pas bon!"),
        Ok(x) => println!("Incroyabe: {}", x),
    }
*/

    let mut test = Vec::new();
    let mut a: Vec<&str> = Vec::new();

    a.push("Dello");
    a.push("Wolrd");
    test.push(a);

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
