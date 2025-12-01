pub fn resolve(s: &str) -> i32 {
    let transformed_data = transform_data(s);
    let result = transformed_data.iter().fold((50, 0), |acc, offset| get_next_value(acc, offset));
    result.1
}

fn transform_data(s: &str) -> Vec<i32> {
    let mut offsets = vec![];

    for l in s.lines() {
        let (direction, offset) = l.split_at(1);
        match direction {
            "L" => offsets.push(-1 * offset.parse::<i32>().unwrap()),
            "R" => offsets.push(1 * offset.parse::<i32>().unwrap()),
            _ => panic!("Unknown direction: {direction}, in line: {l}"),
        }
    }

    offsets
}

fn get_next_value(acc: (i32, i32), offset: &i32) -> (i32, i32) {
    let (start, count) = acc;
    let next = (start + offset).rem_euclid(100);
    if next == 0 {
        (next, count + 1)
    } else {
        (next, count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_expected_data() -> Vec<i32> {
        vec![ -68, -30, 48, -5, 60, -55, -1, -99, 14, -82 ]
    }

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
        let expected = get_expected_data();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_next_value() {
        let v = get_expected_data();
        assert_eq!(get_next_value((50, 0), &v[0]), (82, 0));
        assert_eq!(get_next_value((82, 0), &v[1]), (52, 0));
        assert_eq!(get_next_value((52, 0), &v[2]), (0, 1));
        assert_eq!(get_next_value((0, 1), &v[3]), (95, 1));
        assert_eq!(get_next_value((95, 1), &v[4]), (55, 1));
        assert_eq!(get_next_value((55, 1), &v[5]), (0, 2));
        assert_eq!(get_next_value((0, 2), &v[6]), (99, 2));
        assert_eq!(get_next_value((99, 2), &v[7]), (0, 3));
        assert_eq!(get_next_value((0, 3), &v[8]), (14, 3));
        assert_eq!(get_next_value((14, 3), &v[9]), (32, 3));
    }

    #[test]
    fn test() {
        let v = get_expected_data();
        let t = 50;

        let t = (t + v[0]).rem_euclid(100);
        assert_eq!(t, 82);
        let t = (t + v[1]).rem_euclid(100);
        assert_eq!(t, 52);
        let t = (t + v[2]).rem_euclid(100);
        assert_eq!(t, 0);
        let t = (t + v[3]).rem_euclid(100);
        assert_eq!(t, 95);
        let t = (t + v[4]).rem_euclid(100);
        assert_eq!(t, 55);
        let t = (t + v[5]).rem_euclid(100);
        assert_eq!(t, 0);
        let t = (t + v[6]).rem_euclid(100);
        assert_eq!(t, 99);
        let t = (t + v[7]).rem_euclid(100);
        assert_eq!(t, 0);
        let t = (t + v[8]).rem_euclid(100);
        assert_eq!(t, 14);
        let t = (t + v[9]).rem_euclid(100);
        assert_eq!(t, 32);
    }
}
