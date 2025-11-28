pub mod utils {

    pub fn utils() {
        println!("Hello, world in utils!");
    }

}

pub mod core {

    pub fn parse() -> i32 {
        5
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::core::*;

    #[test]
    fn test_add() {
        assert_eq!(parse(), 5);
    }

    #[test]
    fn test_bad_add() {
        assert_eq!(parse(), 7);
    }
}
