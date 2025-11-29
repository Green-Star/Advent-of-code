pub fn resolve(s: &str) -> i32 {
    let (mut left, mut right) = transform_data(s);
    left.sort();
    right.sort();
    let final_data = left.iter().zip(right.iter()).fold(0, |acc, (a, b)| acc + distance(a, b));

    final_data
}

fn transform_data(s: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];

    for l in s.lines() {
        let numbers = utils::core::parse_number_list(l);
        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    (left, right)
}

fn distance(a: &i32, b: &i32) -> i32 {
    (b - a).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let test_input =
"3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(resolve(test_input), 11);
    }

    #[test]
    fn test_transform_data() {
        let test_input =
"3   4
4   3
2   5
1   3
3   9
3   3";

        let result = transform_data(test_input);
        let expected = (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(result, expected);
    }

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
}
