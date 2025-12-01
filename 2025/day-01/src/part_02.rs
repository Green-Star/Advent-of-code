pub fn resolve(s: &str) -> i32 {
    let mut left = transform_data(s);
    left.sort();
    let final_data = 0;
    final_data
}

fn transform_data(s: &str) -> Vec<i32> {
    let mut left = vec![];
    let mut right = vec![];

    for l in s.lines() {
        let numbers = utils::core::parse_number_list(l);
        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let test_input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

        assert_eq!(resolve(test_input), 3);
    }

    #[test]
    fn test_transform_data() {
        let test_input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

        let result = transform_data(test_input);
        let expected = result.clone();
        assert_eq!(result, expected);
    }

    /*
    #[test]
    fn distance_01() {
        assert_eq!(distance(&1, &3), 2);
    }
    #[test]
    fn distance_02() {
        assert_eq!(distance(&2, &3), 1);
    }
    #[test]
    fn distance_03() {
        assert_eq!(distance(&3, &3), 0);
    }
    #[test]
    fn distance_04() {
        assert_eq!(distance(&3, &4), 1);
    }
    #[test]
    fn distance_05() {
        assert_eq!(distance(&3, &5), 2);
    }
    #[test]
    fn distance_06() {
        assert_eq!(distance(&4, &9), 5);
    }
    */
}
