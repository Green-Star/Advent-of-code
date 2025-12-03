use std::cmp::max;

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

/*
fn get_joltage(bank: &Vec<u32>) -> u64 {
    /* Search the max value for tens in the n-1 elements of the bank (as we will need at least 1 element after this value for the unit part of the joltage) */
    let sliced_bank = &bank[..bank.len()-1];
    let (ten_index, tens) = find_max_in_slice(sliced_bank);

    /* Search the max value for unit (starting with the first element after the tens position) */
    let sliced_bank = &bank[ten_index+1..];
    let (_, unit) = find_max_in_slice(sliced_bank);

    (tens * 10 + unit).into()
}
*/
fn get_joltage(bank: &Vec<u32>) -> u64 {
    let mut joltage: u64 = 0;
    let mut start_index = 0;
    let nb_batteries = 2;
    for power in 0..nb_batteries {
//        let end_index = bank.len() - (nb_batteries - 1 - power);
//        let sliced_bank = &bank[start_index..end_index];
        let remaining_elements = nb_batteries - power - 1;
        let sliced_bank = &bank[start_index..bank.len()-remaining_elements];
        let (max_index, max) = find_max_in_slice(sliced_bank);
        joltage = joltage * 10 + Into::<u64>::into(*max);
        start_index = max_index + 1;
    }

    joltage

    // 1 quand 0
    // 0 quand 1

    // /* Search the max value for tens in the n-1 elements of the bank (as we will need at least 1 element after this value for the unit part of the joltage) */
    // let sliced_bank = &bank[..bank.len()-1];
    // let (ten_index, tens) = find_max_in_slice(sliced_bank);

    // /* Search the max value for unit (starting with the first element after the tens position) */
    // let sliced_bank = &bank[ten_index+1..];
    // let (_, unit) = find_max_in_slice(sliced_bank);

    // (tens * 10 + unit).into()
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

        assert_eq!(resolve(test_input), 357);
    }

    #[test]
    fn test_joltage_01() {
        let bank_test = vec![ 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1 ];
        assert_eq!(get_joltage(&bank_test), 98);
    }
    #[test]
    fn test_joltage_02() {
        let bank_test = vec![ 8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9 ];
        assert_eq!(get_joltage(&bank_test), 89);
    }
    #[test]
    fn test_joltage_03() {
        let bank_test = vec![ 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8 ];
        assert_eq!(get_joltage(&bank_test), 78);
    }
    #[test]
    fn test_joltage_04() {
        let bank_test = vec![ 8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1 ];
        assert_eq!(get_joltage(&bank_test), 92);
    }
}
