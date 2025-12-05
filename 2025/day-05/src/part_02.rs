pub fn resolve(s: &str) -> usize {
    let transformed_data = transform_data(s);
    let sanitized_data = transformed_data.sanitize();
    let final_result = sanitized_data.ingredients.iter().filter(|&&i| sanitized_data.is_fresh(i)).count();
    final_result
}

fn transform_data(data: &str) -> Inventory {
    let mut ranges = vec![];
    let mut ingredients = vec![];

    let mut range_list = true;
    for l in data.lines() {
        if l.is_empty() {
            range_list = false;
            continue;
        }

        if range_list {
            let (range_start, range_end) = utils::core::parse_range(l);
            ranges.push(Range { start: range_start, end: range_end });
        } else {
            ingredients.push(l.parse().unwrap());
        }
    }

    Inventory { ranges, ingredients }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64
}
#[derive(Debug, PartialEq, Eq)]
struct Inventory {
    ranges: Vec<Range>,
    ingredients: Vec<i64>,
}
impl Inventory {
    /* In case it turns out to be usefull to merge the overlapping ranges */
    fn sanitize(&self) -> Self {
        /* Useless for now, simply return the data */
        let ranges = self.ranges.clone();

        Inventory { ranges, ingredients: self.ingredients.clone() }
    }

    fn is_fresh(&self, ingredient: i64) -> bool {
        self.ranges.iter().any(|range| range.start <= ingredient && ingredient <= range.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn easy_setup_data() -> Inventory {
        Inventory {
            ranges: vec![ Range { start: 3, end: 5 }, Range { start: 10, end: 14 }, Range { start: 16, end: 20 },
                        Range { start: 12, end: 18 }, ],
            ingredients: vec![ 1, 5, 8, 11, 17, 32 ],
        }
    }

    #[test]
    fn test_part_02() {
        let test_input = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

        assert_eq!(resolve(test_input), 3);
    }


    #[test]
    fn test_part_transfrom_data() {
        let test_input = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        let expected = easy_setup_data();
        assert_eq!(transform_data(test_input), expected);
    }

    #[test]
    fn test_sanitize_data() {
        let test_data = easy_setup_data();
        let sanitized = test_data.sanitize();

        assert_eq!(sanitized, test_data);
    }
    #[test]
    fn test_fresh_ingredient_01() {
        let test_data = easy_setup_data();
        assert_eq!(test_data.is_fresh(1), false);
    }
    #[test]
    fn test_fresh_ingredient_02() {
        let test_data = easy_setup_data();
        assert_eq!(test_data.is_fresh(5), true);
    }
    #[test]
    fn test_fresh_ingredient_03() {
        let test_data = easy_setup_data();
        assert_eq!(test_data.is_fresh(8), false);
    }
    #[test]
    fn test_fresh_ingredient_04() {
        let test_data = easy_setup_data();
        assert_eq!(test_data.is_fresh(11), true);
    }
    #[test]
    fn test_fresh_ingredient_05() {
        let test_data = easy_setup_data();
        assert_eq!(test_data.is_fresh(17), true);
    }
    #[test]
    fn test_fresh_ingredient_06() {
        let test_data = easy_setup_data();
        assert_eq!(test_data.is_fresh(32), false);
    }
}
