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
        (2..=self_string.len()).any(|nb_substring| Self::check_if_all_substrings_are_equal(Self::split_in_multiple_string(&self_string, nb_substring)))
    }
    fn _is_valid_id(&self) -> bool
    where Self: std::fmt::Display {
        !self.is_invalid_id()
    }


    fn split_in_multiple_string(s: &str, nb_output_strings: usize) -> Vec<Option<&str>> {
        /* If s cannot be equally divide in nb_output_strings, return nothing */
        if s.len() % nb_output_strings != 0 { return vec![ None ] }

        /* Otherise, split s in nb_output_strings (each with the same number of chars) */
        let mut substrings = vec![];

        let substring_length = s.len() / nb_output_strings;
        let mut offset = 0;

        while offset < s.len() {
            substrings.push(s.get(offset..offset + substring_length));
            offset += substring_length;
        }

        substrings
    }
    fn check_if_all_substrings_are_equal(substrings: Vec<Option<&str>>) -> bool {
        substrings.iter().all(|o| o.is_some()) && substrings.windows(2).all(|w| w[0] == w[1])
    }
}
impl ID for i64 {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_02() {
        let test_input = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124\
";

        assert_eq!(resolve(test_input), 4174379265);
    }

    #[test]
    fn test_trait_for_id() {
        assert_eq!(11_i64.is_invalid_id(), true);
        assert_eq!(20_i64.is_invalid_id(), false);
        assert_eq!(22_i64.is_invalid_id(), true);
    }
    #[test]
    fn test_some_ids() {
        assert_eq!(11_i64.is_invalid_id(), true);
        assert_eq!(20_i64.is_invalid_id(), false);
        assert_eq!(22_i64.is_invalid_id(), true);
        assert_eq!(12341234_i64.is_invalid_id(), true);
        assert_eq!(123123123_i64.is_invalid_id(), true);
        assert_eq!(1212121212_i64.is_invalid_id(), true);
        assert_eq!(1111111_i64.is_invalid_id(), true);
    }

    #[test]
    fn test_invalid_ids_in_range_01() {
        let range = Range { start: 11, end: 22 };
        assert_eq!(range.get_invalid_ids(), Some(vec![ 11, 22 ]));
    }
    #[test]
    fn test_invalid_ids_in_range_02() {
        let range = Range { start: 95, end: 115 };
        assert_eq!(range.get_invalid_ids(), Some(vec![ 99, 111 ]));
    }
    #[test]
    fn test_invalid_ids_in_range_03() {
        let range = Range { start: 998, end: 1012 };
        assert_eq!(range.get_invalid_ids(), Some(vec![ 999, 1010 ]));
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
        assert_eq!(range.get_invalid_ids(), Some(vec![ 565656 ]));
    }
    #[test]
    fn test_invalid_ids_in_range_10() {
        let range = Range { start: 824824821, end: 824824827 };
        assert_eq!(range.get_invalid_ids(), Some(vec![ 824824824 ]));
    }
    #[test]
    fn test_invalid_ids_in_range_11() {
        let range = Range { start: 2121212118, end: 2121212124 };
        assert_eq!(range.get_invalid_ids(), Some(vec![ 2121212121 ]));
    }

    #[test]
    fn test_substring_01() {
        assert_eq!(<i64 as ID>::split_in_multiple_string("12341234", 2), vec![ Some("1234"), Some("1234") ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("12341234", 3), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("12341234", 4), vec![ Some("12"), Some("34"), Some("12"), Some("34") ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("12341234", 5), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("12341234", 6), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("12341234", 7), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("12341234", 8), vec![ Some("1"), Some("2"), Some("3"), Some("4"), Some("1"), Some("2"), Some("3"), Some("4") ]);
    }
    #[test]
    fn test_validate_substring_01() {
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ Some("1234"), Some("1234") ]), true);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ Some("12"), Some("34"), Some("12"), Some("34") ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ Some("1"), Some("2"), Some("3"), Some("4"), Some("1"), Some("2"), Some("3"), Some("4") ]), false);
    }
    #[test]
    fn test_substring_02() {
        assert_eq!(<i64 as ID>::split_in_multiple_string("2121212121", 2), vec![ Some("21212"), Some("12121") ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("2121212121", 3), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("2121212121", 4), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("2121212121", 5), vec![ Some("21"), Some("21"), Some("21"), Some("21"), Some("21") ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("2121212121", 6), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("2121212121", 7), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("2121212121", 8), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("2121212121", 9), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("2121212121", 10), vec![ Some("2"), Some("1"), Some("2"), Some("1"), Some("2"), Some("1"), Some("2"), Some("1"), Some("2"), Some("1") ]);
    }
    #[test]
    fn test_validate_substring_02() {
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ Some("21212"), Some("12121") ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ Some("21"), Some("21"), Some("21"), Some("21"), Some("21") ]), true);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ Some("2"), Some("1"), Some("2"), Some("1"), Some("2"), Some("1"), Some("2"), Some("1"), Some("2"), Some("1") ]), false);
    }
    #[test]
    fn test_substring_03() {
        assert_eq!(<i64 as ID>::split_in_multiple_string("1111111", 2), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("1111111", 3), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("1111111", 4), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("1111111", 5), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("1111111", 6), vec![ None ]);
        assert_eq!(<i64 as ID>::split_in_multiple_string("1111111", 7), vec![ Some("1"), Some("1"), Some("1"), Some("1"), Some("1"), Some("1"), Some("1") ]);
    }
    #[test]
    fn test_validate_substring_03() {
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ None ]), false);
        assert_eq!(<i64 as ID>::check_if_all_substrings_are_equal(vec![ Some("1"), Some("1"), Some("1"), Some("1"), Some("1"), Some("1"), Some("1") ]), true);
    }
}
