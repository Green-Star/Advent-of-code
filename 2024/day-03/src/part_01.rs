use std::i32;
use regex::Regex;

pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let products = transform_data(data);

    let final_result: i32 = products.iter().map(|e| e.value()).sum();

    println!("Part 1 final result: {}", final_result);
}

#[derive(Debug, Clone, Copy)]
struct Product {
    x: i32,
    y: i32,
}
impl Product {
    fn value(&self) -> i32 {
        self.x * self.y
    }
}

fn transform_data(data: Vec<String>) -> Vec<Product> {
    let mut result = Vec::new();
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for line in data {
        for (_, [a, b]) in regex.captures_iter(&line).map(|r| r.extract()) {
            let x = a.parse().unwrap();
            let y = b.parse().unwrap();
            result.push(Product { x, y });
        }
    }

    result
}
