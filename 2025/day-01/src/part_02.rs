pub fn resolve(s: &str) -> i32 {
    let (left, right) = transform_data(s);
    let final_result = left.iter().fold(0, |acc, x| acc + similarity_score(x, &right));

    final_result
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

fn similarity_score(x: &i32, vec: &Vec<i32>) -> i32 {
    let occurences = vec.iter().filter(|y| x == *y).count();
    x * (occurences as i32)
}

fn distance(a: &i32, b: &i32) -> i32 {
    (b - a).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_02() {
        let test_input =
"3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(resolve(test_input), 31);
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

    fn get_test_data() -> Vec<i32> {
        vec![4, 3, 5, 3, 9, 3]
    }

    #[test]
    fn similarity_score_01() {
        let data = get_test_data();
        assert_eq!(similarity_score(&3, &data), 9);
    }
    #[test]
    fn similarity_score_02() {
        let data = get_test_data();
        assert_eq!(similarity_score(&4, &data), 4);
    }
    #[test]
    fn similarity_score_03() {
        let data = get_test_data();
        assert_eq!(similarity_score(&2, &data), 0);
    }
    #[test]
    fn similarity_score_04() {
        let data = get_test_data();
        assert_eq!(similarity_score(&1, &data), 0);
    }
    #[test]
    fn similarity_score_05() {
        let data = get_test_data();
        assert_eq!(similarity_score(&3, &data), 9);
    }
    #[test]
    fn similarity_score_06() {
        let data = get_test_data();
        assert_eq!(similarity_score(&3, &data), 9);
    }
}
