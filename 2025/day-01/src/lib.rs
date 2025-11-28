
pub fn test() -> bool {
    true
}

pub fn part_01() -> i32 {
    10
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(test(), true);
    }

    #[test]
    fn test_bad_add() {
        assert_eq!(part_01(), 10);
    }
}
