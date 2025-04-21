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

pub fn parse_number_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    parse_number_list_with_separator(s, ",")
}

pub fn parse_number_list_with_separator<T: std::str::FromStr>(s: &str, sep: &str) -> Vec<T> {
    s.split(sep).filter_map(|s| s.parse::<T>().ok()).collect()
}
