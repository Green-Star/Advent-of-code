use std::i32;
use regex::Regex;

pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let products = transform_data(data);
//    let products = get_multiplications(data);

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

/*
fn transform_data(data: Vec<String>) -> Vec<Product> {
    let mut result = Vec::new();
    let mut enabled = true;

    for line in data {
        /* Parse les don't() */
            /* Pour chaque don't, on cherche un do() et on skip ce qu'il y entre les 2 */
        /*  */
        for (_, [a, b]) in regex.captures_iter(&line).map(|r| r.extract()) {
            let x = a.parse().unwrap();
            let y = b.parse().unwrap();
            result.push(Product { x, y });
        }
    }

    result
}
    */

fn transform_data(data: Vec<String>) -> Vec<Product> {
    let mut result = Vec::new();
    let mut enabled = true;

    let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let mult_regex = Regex::new(r"\((\d{1,3}),(\d{1,3})\)").unwrap();

    for line in data {
        for (head, [matched]) in regex.captures_iter(&line).map(|r| r.extract()) {
            match head {
                "do()" => { println!("Found do !"); enabled = true },
                "don't()" => { println!("Found don't !"); enabled = false },
                _ => {
                    if enabled == false { continue; }
                    println!("Found mult !");
                    for (_, [a, b]) in mult_regex.captures_iter(&matched).map(|r| r.extract()) {
                        let x = a.parse().unwrap();
                        let y = b.parse().unwrap();
                        result.push(Product { x, y });
                    }
                }
            }
        }
    }

    result
}
