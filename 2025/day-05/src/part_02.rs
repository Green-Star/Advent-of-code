use std::cmp::max;

pub fn resolve(s: &str) -> i64 {
    let transformed_data = transform_data(s);
    let sanitized_data = transformed_data.sanitize();
    let final_result = sanitized_data.get_all_fresh_ingredients();
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
    /* In case it turns out to be useful to merge the overlapping ranges */
    /* It turned out to be useful :) */
    fn sanitize(&self) -> Self {
        let mut sanitized_ranges = self.ranges.clone();
        let mut merged = true;

        while merged {
            let mut ranges_to_be_processed: Vec<_> = sanitized_ranges.iter().map(|r| SanitizedRange { range: *r, processed: false }).collect();
            sanitized_ranges = vec![];
            merged = false;

            for i in 0..ranges_to_be_processed.len() {
                if ranges_to_be_processed[i].processed { continue; }

                let mut merged_max = ranges_to_be_processed[i].range.end;
                for j in i+1..ranges_to_be_processed.len() {
                    if ranges_to_be_processed[j].range.start < ranges_to_be_processed[i].range.end {
                        merged_max = max(merged_max, ranges_to_be_processed[j].range.end);
                        ranges_to_be_processed[j].processed = true;
                        merged = true;
                    }
                }

                sanitized_ranges.push( Range {start: ranges_to_be_processed[i].range.start, end: merged_max });
                ranges_to_be_processed[i].processed = true;
            }
        }

        /*
        let mut ranges_to_be_processed: Vec<_> = self.ranges.iter().map(|r| SanitizedRange { range: *r, processed: false }).collect();

        let mut sanitized_ranges = vec![];

        for i in 0..ranges_to_be_processed.len() {
            if ranges_to_be_processed[i].processed { continue; }

            let mut merged_max = ranges_to_be_processed[i].range.end;
            for j in i+1..ranges_to_be_processed.len() {
                if ranges_to_be_processed[j].range.start < ranges_to_be_processed[i].range.end {
                    merged_max = max(merged_max, ranges_to_be_processed[j].range.end);
                    ranges_to_be_processed[j].processed = true;
                    merged = true;
                }
            }

            sanitized_ranges.push( Range {start: ranges_to_be_processed[i].range.start, end: merged_max });
            ranges_to_be_processed[i].processed = true;
        }
        */

/*
        for (index, range) in self.ranges.iter().enumerate() {
            for r in &self.ranges[index..] {
                if r.start < range.end {
                    Range { start: range.start, end: max(r.end, range.end) };
                }
            }
        }
*/

        Inventory { ranges: sanitized_ranges, ingredients: self.ingredients.clone() }
    }

    fn is_fresh(&self, ingredient: i64) -> bool {
        self.ranges.iter().any(|range| range.start <= ingredient && ingredient <= range.end)
    }

    fn get_all_fresh_ingredients(&self) -> i64 {
        self.ranges.iter().map(|r| self.get_number_of_fresh_ingredients(r)).sum()
    }
    fn get_number_of_fresh_ingredients(&self, range: &Range) -> i64 {
        range.end - range.start + 1
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct SanitizedRange {
    range: Range,
    processed: bool,
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

        assert_eq!(sanitized, Inventory {
                                ranges: vec![ Range { start: 3, end: 5 }, Range { start: 10, end: 20 }, ],
                                ingredients: vec![ 1, 5, 8, 11, 17, 32 ],
        });
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
