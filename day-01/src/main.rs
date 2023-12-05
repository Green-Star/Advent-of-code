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




fn main() {
    println!("Hello, world in proper Rust!");

    let data = load_file_in_memory("./test.data").unwrap();

    for s in data {
        println!("Line loaded: [{}]", s);
    }
}
