pub fn resolve(s: &str) -> i32 {
    let transformed_data = transform_data(s);
    0
}

fn transform_data(data: &str) -> Vec<i32> {
    let mut result = vec![];

    for l in data.lines() {
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let test_input = "\
0
";

        assert_eq!(resolve(test_input), 0);
    }
}
