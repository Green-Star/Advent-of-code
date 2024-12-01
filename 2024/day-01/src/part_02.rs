use std::{collections::HashMap};

pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let (left_locations, right_locations) = transform_data(data);
    let mut location_map = get_locations_map(left_locations);
    get_similarity(right_locations, &mut location_map);

    let final_result = location_map.into_values().fold(0, |sum, entry| sum + entry.get_score());

    println!("Part 2 final result: {}", final_result);
}

#[derive(Debug, Clone, Copy)]
struct SimilarityScore {
    id: i32,
    occurences: i32,
    similarity_factor: i32,
}
impl SimilarityScore {
    fn get_score(&self) -> i32  {
        self.id * self.occurences * self.similarity_factor
    }
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

fn get_locations_map(location_list: Vec<i32>) -> HashMap<i32, SimilarityScore> {
    let mut result = HashMap::new();

    for l in location_list {
        result.entry(l).and_modify(|entry: &mut SimilarityScore| { entry.occurences += 1 }).or_insert(SimilarityScore { id: l, occurences: 1, similarity_factor: 0 });
    }

    result
}

fn get_similarity(location_list: Vec<i32>, location_map: &mut HashMap<i32, SimilarityScore>) {
    for location in location_list {
        location_map.entry(location).and_modify(|entry| { entry.similarity_factor += 1 });
    }
}
