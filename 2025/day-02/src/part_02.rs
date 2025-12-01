pub fn resolve(s: &str) -> i32 {
    let transformed_data = transform_data(s);
    0
}

fn transform_data(s: &str) -> Vec<i32> {
    let v = vec![];

    for l in s.lines() {

    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_expected_data() -> Vec<i32> {
        vec![]
    }

    #[test]
    fn test_part_01() {
        let test_input = "\
";

        assert_eq!(resolve(test_input), 0);
    }

    #[test]
    fn test_transform_data() {
        let test_input = "\
";

        let result = transform_data(test_input);
        let expected = get_expected_data();
        assert_eq!(result, expected);
    }
}
