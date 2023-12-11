use std::cmp::min;
use std::fs::File;
use std::i128;
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


trait InsideExt: PartialOrd {
    fn is_inside_right_open_interval(&self, start: &Self, end: &Self) -> bool {
        if start <= self && self < end { true } else { false }
    }
}
impl InsideExt for i128 {}

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
    fn transform(&self, x: &i128) -> Result<i128, ()> {
        if x.is_inside_right_open_interval(&self.source, &(self.source + self.range)) { return Ok(self.destination + x - self.source) }
        Err(())
    }
}

#[derive(Debug, Clone)]
struct Almanac {
    entries: Vec<AlmanacLines>,
}
impl std::fmt::Display for Almanac {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Almanac(").unwrap();
        for e in &self.entries { write!(f, "{}", e).unwrap(); };
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
    let mut seeds = Vec::new();
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
    let data = load_file_in_memory("./input-01.data").unwrap();
    let (seeds, almanac_list) = transform_data(data);
    let locations: Vec<i128> = seeds.into_iter().map(|seed| almanac_list.iter().fold(seed, |seed, almanac| almanac.transform(seed))).collect();
    let final_result = locations.iter().reduce(|location_min, location| min(location_min, location)).unwrap();

    println!("Part 1 final result: {}", final_result);
}



fn part_02() {
    let data = load_file_in_memory("./test-02.data").unwrap();
    let (seeds_description, almanac_list) = transform_data(data);

    let mut location_min = None;
    let mut seeds_list = seeds_description.into_iter();
    let final_result = loop {
        match (seeds_list.next(), seeds_list.next()) {
            (Some(seed), Some(range)) => {
                for current_seed in seed .. seed + range {
                    let location = almanac_list.iter().fold(current_seed, |transformed_seed, almanac| almanac.transform(transformed_seed));
                    match location_min {
                        None => location_min = Some(location),
                        Some(current_min) => location_min = Some(min(current_min, location)),
                    }
                }
            }
            (_, _) => break location_min.unwrap()
        }
    };

    println!("Part 2 final result: {}", final_result);
}

fn main() {
    part_01();
    part_02();
}
