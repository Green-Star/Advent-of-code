pub fn resolve() -> i32 {
    utils::core::say_hello();

    10
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_01() {
        assert_eq!(resolve(), 10);
    }
}
