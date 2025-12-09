use std::cmp::Ordering;

pub fn resolve(s: &str) -> usize {
    let transformed_data = transform_data(s);
    0
}

fn transform_data(data: &str) -> Vec<usize> {
    let mut result = vec![];
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_02() {
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

        assert_eq!(resolve(test_input), 0);
    }
}
