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

    /* Strip away the full rotation of the dial */
    let full_rotation = offset.abs() / 100;
    let offset = offset % 100;

    /* Get the final position of the dial and check if we point to 0 to reach it (note: if we started on 0, there is no click to add) */
    let finish = start + offset;
    let going_through_0 = if start > 0 && finish >= 100 { 1 } else if start > 0 && finish <= 0 { 1 } else { 0 };

    /* Finally, get the next value */
    let next = (start + offset).rem_euclid(100);
    (next, count + full_rotation + going_through_0)
}

/*
fn get_next_value(acc: (i32, i32), offset: &i32) -> (i32, i32) {
    let (start, count) = acc;

    if offset >= &0 {
        let x = start + offset;
        (x.rem_euclid(100), count + x.div_euclid(100))
    } else {
        let next = (start + offset).rem_euclid(100);
        (next, count)
    }
/*
    let next = (start + offset).rem_euclid(100);
    if next == 0 {
        (next, count + 1)
    } else {
        (next, count)
    }
    */
}
*/

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

        assert_eq!(resolve(test_input), 6);
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
        assert_eq!(get_next_value((50, 0), &v[0]), (82, 1));
        assert_eq!(get_next_value((82, 1), &v[1]), (52, 1));
        assert_eq!(get_next_value((52, 1), &v[2]), (0, 2));
        assert_eq!(get_next_value((0, 2), &v[3]), (95, 2));
        assert_eq!(get_next_value((95, 2), &v[4]), (55, 3));
        assert_eq!(get_next_value((55, 3), &v[5]), (0, 4));
        assert_eq!(get_next_value((0, 4), &v[6]), (99, 4));
        assert_eq!(get_next_value((99, 4), &v[7]), (0, 5));
        assert_eq!(get_next_value((0, 5), &v[8]), (14, 5));
        assert_eq!(get_next_value((14, 5), &v[9]), (32, 6));
    }
    #[test]
    fn test_next_value_detail_01() {
        assert_eq!(get_next_value((50, 0), &-68), (82, 1));
    }
    #[test]
    fn test_next_value_detail_02() {
        assert_eq!(get_next_value((82, 1), &-30), (52, 1));
    }
    #[test]
    fn test_next_value_detail_03() {
        assert_eq!(get_next_value((52, 1), &48), (0, 2));
    }
    #[test]
    fn test_next_value_detail_04() {
        assert_eq!(get_next_value((0, 2), &-5), (95, 2));
    }
    #[test]
    fn test_next_value_detail_05() {
        assert_eq!(get_next_value((95, 2), &60), (55, 3));
    }
    #[test]
    fn test_next_value_detail_06() {
        assert_eq!(get_next_value((55, 3), &-55), (0, 4));
    }
    #[test]
    fn test_next_value_detail_07() {
        assert_eq!(get_next_value((0, 4), &-1), (99, 4));
    }
    #[test]
    fn test_next_value_detail_08() {
        assert_eq!(get_next_value((99, 4), &-99), (0, 5));
    }
    #[test]
    fn test_next_value_detail_09() {
        assert_eq!(get_next_value((0, 5), &14), (14, 5));
    }
    #[test]
    fn test_next_value_detail_10() {
        assert_eq!(get_next_value((14, 5), &-82), (32, 6));
    }

    #[test]
    fn test_next_value_detail_11() {
        assert_eq!(get_next_value((0, 0), &-18), (82, 0));
    }
    #[test]
    fn test_next_value_detail_12() {
        assert_eq!(get_next_value((0, 0), &18), (18, 0));
    }

    #[test]
    fn test_full_rotation() {
        assert_eq!(get_next_value((50, 0), &1000), (50, 10));
        assert_eq!(get_next_value((50, 0), &-1000), (50, 10));

        assert_eq!(get_next_value((50, 0), &1000), (50, 10));
        assert_eq!(get_next_value((50, 10), &-1000), (50, 20));
    }

    #[test]
    fn test_full_rotation_02() {
        assert_eq!(get_next_value((0, 0), &100), (0, 1));
        assert_eq!(get_next_value((0, 0), &-100), (0, 1));

        assert_eq!(get_next_value((0, 0), &1000), (0, 10));
        assert_eq!(get_next_value((0, 0), &-1000), (0, 10));

        assert_eq!(get_next_value((0, 0), &1000), (0, 10));
        assert_eq!(get_next_value((0, 10), &-1000), (0, 20));
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
