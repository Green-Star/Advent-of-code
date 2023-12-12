use std::cmp::min;
use std::fs::File;
use std::i128;
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

/* Input string examples:
    [ 41 48 83 86 17 ]
    [ 83 86  6 31 17  9 48 53]

    Result: Vec of numbers
*/
fn parse_number_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.split(" ").filter_map(|s| s.parse::<T>().ok()).collect()
}

fn part_01() {
    let data = load_file_in_memory("./test-01.data").unwrap();

    let final_result = 0;

    println!("Part 1 final result: {}", final_result);
}

fn part_02() {

}


fn main() {
    let now = Instant::now();
    part_01();
    let elapsed = now.elapsed();
    println!("Part 1 found in {:?}s", elapsed.as_secs());
    let now = Instant::now();
    part_02();
    let elapsed = now.elapsed();
    println!("Optimized part 2 found in {:?}s", elapsed.as_secs());
}
