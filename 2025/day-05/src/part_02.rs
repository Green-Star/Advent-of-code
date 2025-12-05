pub fn resolve(s: &str) -> usize {
    let transformed_data = transform_data(s);
    0
}

fn transform_data(data: &str) -> Vec<i32> {
    let result = vec![];

    for l in data.lines() {
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_02() {
        let test_input = "\
";

        assert_eq!(resolve(test_input), 0);
    }
}
