pub fn resolve(s: &str) -> i64 {
    let transformed_data = transform_data(s);
    let invalid_ids: Vec<i64> = transformed_data.iter()
                                        .filter_map(|range| range.get_invalid_ids())
                                        .flatten()
                                        .collect();
    invalid_ids.iter().sum()
}

fn transform_data(data: &str) -> Vec<Range> {
    let mut result = vec![];

    for l in data.lines() {
        l.split(",").for_each(|s| {
            let numbers = utils::core::parse_number_list_with_separator(s, "-");
            result.push( Range { start: numbers[0], end: numbers[1] } );
        });
    }

    result
}

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64
}
impl Range {
    fn get_invalid_ids(&self) -> Option<Vec<i64>> {
        let invalid_ids: Vec<_> = (self.start ..= self.end).filter(|x| x.is_invalid_id()).collect();
        if invalid_ids.is_empty() { None } else { Some(invalid_ids) }
    }
}

trait ID {
    fn is_invalid_id(&self) -> bool
    where Self: std::fmt::Display {
        let self_string = format!("{self}");
        let (left, right) = self_string.split_at(self_string.len() / 2);
        left == right
    }
    fn is_valid_id(&self) -> bool
    where Self: std::fmt::Display {
        !self.is_invalid_id()
    }
}
impl ID for i64 {}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_expected_data() -> Vec<Range> {
        vec![ Range { start: 11, end: 22 }, Range { start: 95, end: 115 },
            Range { start: 998, end: 1012 }, Range { start: 1188511880, end: 1188511890 },
            Range { start: 222220, end: 222224 }, Range { start: 1698522, end: 1698528 },
            Range { start: 446443, end: 446449 }, Range { start: 38593856, end: 38593862 },
            Range { start: 565653, end: 565659 }, Range { start: 824824821, end: 824824827 },
            Range { start: 2121212118, end: 2121212124 },
        ]
    }

    #[test]
    fn test_part_01() {
        let test_input = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124\
";

        assert_eq!(resolve(test_input), 1227775554);
    }

    #[test]
    fn test_trait_for_id() {
        assert_eq!(11_i64.is_invalid_id(), true);
        assert_eq!(20_i64.is_invalid_id(), false);
        assert_eq!(22_i64.is_invalid_id(), true);
        assert_eq!(20_i64.is_valid_id(), true);
        assert_eq!(22_i64.is_valid_id(), false);
    }

    #[test]
    fn test_invalid_ids_in_range_01() {
        let range = Range { start: 11, end: 22 };
        assert_eq!(range.get_invalid_ids(), Some(vec![ 11, 22 ]));
    }
    #[test]
    fn test_invalid_ids_in_range_02() {
        let range = Range { start: 95, end: 115 };
        assert_eq!(range.get_invalid_ids(), Some(vec![ 99 ]));
    }
    #[test]
    fn test_invalid_ids_in_range_03() {
        let range = Range { start: 998, end: 1012 };
        assert_eq!(range.get_invalid_ids(), Some(vec![ 1010 ]));
    }
    #[test]
    fn test_invalid_ids_in_range_04() {
        let range = Range { start: 1188511880, end: 1188511890 };
        assert_eq!(range.get_invalid_ids(), Some(vec![ 1188511885 ]));
    }
    #[test]
    fn test_invalid_ids_in_range_05() {
        let range = Range { start: 222220, end: 222224 };
        assert_eq!(range.get_invalid_ids(), Some(vec![ 222222 ]));
    }
    #[test]
    fn test_invalid_ids_in_range_06() {
        let range = Range { start: 1698522, end: 1698528 };
        assert_eq!(range.get_invalid_ids(), None);
    }
    #[test]
    fn test_invalid_ids_in_range_07() {
        let range = Range { start: 446443, end: 446449 };
        assert_eq!(range.get_invalid_ids(), Some(vec![ 446446 ]));
    }
    #[test]
    fn test_invalid_ids_in_range_08() {
        let range = Range { start: 38593856, end: 38593862 };
        assert_eq!(range.get_invalid_ids(), Some(vec![ 38593859 ]));
    }
    #[test]
    fn test_invalid_ids_in_range_09() {
        let range = Range { start: 565653, end: 565659 };
        assert_eq!(range.get_invalid_ids(), None);
    }
    #[test]
    fn test_invalid_ids_in_range_10() {
        let range = Range { start: 824824821, end: 824824827 };
        assert_eq!(range.get_invalid_ids(), None);
    }
    #[test]
    fn test_invalid_ids_in_range_11() {
        let range = Range { start: 2121212118, end: 2121212124 };
        assert_eq!(range.get_invalid_ids(), None);
    }
}
