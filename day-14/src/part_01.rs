
pub fn resolve(input_data_path: &str) {
//    let data = crate::core::load_file_in_memory(input_data_path).unwrap();

    let final_result = 0;

    println!("Part 1 final result: {}", final_result);
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

fn create_vector() -> Vec<Vec<i32>> {
    vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![10, 11, 12, 13],
    ]
}

fn create_other_vector() -> Vec<Vec<i32>> {
    vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {

    }

    #[test]
    fn internal() {
        setup();
        assert_eq!(4, internal_adder(2, 2));
    }

    #[test]
    fn test_1() {
        let result = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![10, 11, 12, 13],
        ];

        let v = create_vector();

        assert_eq!(v, result);
    }

    #[test]
    fn test2() {
        let result = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![10, 11, 12, 13],
        ];

        let diferent_vector = create_other_vector();

        assert_ne!(diferent_vector, result);
    }
}
