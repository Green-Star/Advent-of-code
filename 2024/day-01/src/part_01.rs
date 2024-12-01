pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let (left_locations, right_locations) = transform_data(data);
    let pairs = pair_locations(left_locations, right_locations);

    let final_result = pairs.iter().fold(0, |sum, (x, y)| sum + get_distance(x, y));

    println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for s in data {
        let mut matched = s.split_whitespace();
        let left = matched.next().unwrap().parse().unwrap();
        let right = matched.next().unwrap().parse().unwrap();

        left_list.push(left);
        right_list.push(right);
    }

    (left_list, right_list)
}

fn pair_locations(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<(i32, i32)> {
    left.sort();
    right.sort();

    let mut result = Vec::new();

    for i in 0..left.len() {
        result.push( (left[i], right[i]) );
    }

    result
}

fn get_distance(a: &i32, b: &i32) -> i32 {
    (a - b).abs()
}
