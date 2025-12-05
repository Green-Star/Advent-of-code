use std::cmp::{max, min};

pub fn resolve(s: &str) -> i64 {
    let transformed_data = transform_data(s);
    let sanitized_data = transformed_data.sanitize();
    let final_result = sanitized_data.get_all_fresh_ingredients();
    final_result
}

fn transform_data(data: &str) -> Inventory {
    let mut ranges = vec![];
    let mut _ingredients: Vec<i64> = vec![];

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
            _ingredients.push(l.parse().unwrap());
        }
    }

    Inventory { ranges }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64
}
#[derive(Debug, PartialEq, Eq)]
struct Inventory {
    ranges: Vec<Range>,
}
impl Inventory {
    /* In case it turns out to be useful to merge the overlapping ranges */
    /* It turned out to be useful :) */
    fn sanitize(&self) -> Self {
        let mut sanitized_ranges = self.ranges.clone();
        let mut merged = true;

        while merged {
            let ranges_to_be_processed: Vec<_> = sanitized_ranges;
            sanitized_ranges = vec![];
            merged = false;

            for to_be_processed in ranges_to_be_processed {
                match sanitized_ranges.iter_mut().find(|range| (to_be_processed.start <= range.start && to_be_processed.end >= range.start) || (range.start <= to_be_processed.start && range.end >= to_be_processed.start)) {
                    Some(range) => { range.start = min(range.start, to_be_processed.start); range.end = max(range.end, to_be_processed.end); merged = true },
                    None => sanitized_ranges.push(to_be_processed),
                }
            }
        }

        Inventory { ranges: sanitized_ranges }
    }

    fn get_all_fresh_ingredients(&self) -> i64 {
        self.ranges.iter().map(|r| self.get_number_of_fresh_ingredients(r)).sum()
    }
    fn get_number_of_fresh_ingredients(&self, range: &Range) -> i64 {
        range.end - range.start + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn easy_setup_data() -> Inventory {
        Inventory {
            ranges: vec![
                        Range { start: 3, end: 5 },
                        Range { start: 10, end: 14 },
                        Range { start: 16, end: 20 },
                        Range { start: 12, end: 18 },
                    ]
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

        assert_eq!(resolve(test_input), 14);
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

        assert_eq!(sanitized, Inventory { ranges: vec![ Range { start: 3, end: 5 }, Range { start: 10, end: 20 } ],
        });
    }

    #[test]
    fn test_sanitize_data_01() {
        let test_data = Inventory { ranges: vec![ Range { start: 1, end: 3 }, Range { start: 6, end: 8 } ] };
        let sanitized = test_data.sanitize();
        assert_eq!(sanitized, Inventory { ranges: vec![ Range { start: 1, end: 3 }, Range { start: 6, end: 8 } ] });
    }

    #[test]
    fn test_sanitize_data_02() {
        let test_data = Inventory { ranges: vec![ Range { start: 1, end: 5 }, Range { start: 4, end: 8 } ] };
        let sanitized = test_data.sanitize();
        assert_eq!(sanitized, Inventory { ranges: vec![ Range { start: 1, end: 8 } ] });
    }

    #[test]
    fn test_sanitize_data_03() {
        let test_data = Inventory { ranges: vec![ Range { start: 1, end: 8 }, Range { start: 3, end: 5 } ] };
        let sanitized = test_data.sanitize();
        assert_eq!(sanitized, Inventory { ranges: vec![ Range { start: 1, end: 8 } ] });
    }

    #[test]
    fn test_sanitize_data_04() {
        let test_data = Inventory { ranges: vec![ Range { start: 3, end: 8 }, Range { start: 1, end: 5 } ] };
        let sanitized = test_data.sanitize();
        assert_eq!(sanitized, Inventory { ranges: vec![ Range { start: 1, end: 8 } ] });
    }

    #[test]
    fn test_sanitize_data_05() {
        let test_data = Inventory { ranges: vec![ Range { start: 6, end: 9 }, Range { start: 1, end: 5 } ] };
        let sanitized = test_data.sanitize();
        assert_eq!(sanitized, Inventory { ranges: vec![ Range { start: 6, end: 9 }, Range { start: 1, end: 5 } ] });
    }

    #[test]
    fn test_fresh_range_01() {
        let test_data = easy_setup_data();

        let test = test_data.get_number_of_fresh_ingredients(&Range { start: 3, end: 5 });
        assert_eq!(test, 3);
    }
    #[test]
    fn test_fresh_range_02() {
        let test_data = easy_setup_data();

        let test = test_data.get_number_of_fresh_ingredients(&Range { start: 10, end: 20 });
        assert_eq!(test, 11);
    }
}
