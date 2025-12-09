pub fn resolve(s: &str) -> usize {
    let transformed_data = transform_data(s);
    let rectangles = create_all_rectangles(&transformed_data);
    let final_result = rectangles.iter().map(|(a, b)| (b.0.abs_diff(a.0) + 1) * (b.1.abs_diff(a.1) + 1)).max().unwrap();
    final_result
}

fn transform_data(data: &str) -> Vec<(usize, usize)> {
    let mut result = vec![];

    for l in data.lines() {
        let pos = utils::core::parse_comma_number_list(l);
        result.push((pos[0], pos[1]));
    }

    result
}

fn create_all_rectangles(vertexes: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    let mut rectangles = vec![];

    for (index, a) in vertexes.iter().enumerate() {
        for b in &vertexes[index+1..] {
            rectangles.push((*a, *b));
        }
    }

    rectangles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let test_input = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

        assert_eq!(resolve(test_input), 50);
    }
}
