pub fn resolve(s: &str) -> usize {
    let transformed_data = transform_data(s);
    0
}

fn transform_data(data: &str) -> Inventory {
    let ranges = vec![];
    let ingredients = vec![];

    for l in data.lines() {
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
    fn sanitize_data(&self) -> Self {
        /* Useless for now, simply return the data */
        let ranges = self.ranges.clone();

        Inventory { ranges, ingredients: self.ingredients.clone() }
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
    fn test_part_01() {
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
        let sanitized = test_data.sanitize_data();

        assert_eq!(sanitized, test_data);
    }
}
