pub fn resolve(s: &str) -> u64 {
    let transformed_data = transform_data(s);
    let final_result: u64 = transformed_data.iter().map(|bank| get_joltage(bank)).sum();
    final_result
}

fn transform_data(data: &str) -> Vec<Vec<u32>> {
    let mut result = vec![];

    for s in data.lines() {
        let mut bank = vec![];
        for c in s.chars() {
            bank.push(c.to_digit(10).unwrap());
        }
        result.push(bank);
    }

    result
}

fn get_joltage(bank: &Vec<u32>) -> u64 {
    let nb_batteries = 12;

    let mut joltage: u64 = 0;
    let mut start_index = 0;
    for power in 0..nb_batteries {
        /* remaining elements in the bank which won't be used this time:
            nb_batteries (the number of elements we need to get from the bank)
            -power (the number of elements we already get in the bank)
            -1 (to stay in the limit of the original bank)
        */
        let remaining_elements = nb_batteries - power - 1;
        let sliced_bank = &bank[start_index..bank.len()-remaining_elements];
        let (max_index, max) = find_max_in_slice(sliced_bank);
        joltage = joltage * 10 + Into::<u64>::into(*max);
        start_index = start_index + max_index + 1;
    }

    joltage
}
fn find_max_in_slice(slice: &[u32]) -> (usize, &u32) {
    slice.iter().enumerate().max_by(|(_a_index, a), (_b_index, b)| if b > a { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater } ).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_02() {
        let test_input = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

        assert_eq!(resolve(test_input), 3121910778619);
    }

    #[test]
    fn test_joltage_01() {
        let bank_test = vec![ 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1 ];
        assert_eq!(get_joltage(&bank_test), 987654321111);
    }
    #[test]
    fn test_joltage_02() {
        let bank_test = vec![ 8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9 ];
        assert_eq!(get_joltage(&bank_test), 811111111119);
    }
    #[test]
    fn test_joltage_03() {
        let bank_test = vec![ 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8 ];
        assert_eq!(get_joltage(&bank_test), 434234234278);
    }
    #[test]
    fn test_joltage_04() {
        let bank_test = vec![ 8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1 ];
        assert_eq!(get_joltage(&bank_test), 888911112111);
    }
}
