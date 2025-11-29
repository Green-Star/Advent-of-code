pub mod core {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn say_hello() {
        println!("Hello world from the utils!");
    }

    pub fn load_file_in_memory(filepath: &str) -> std::io::Result<Vec<String>> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);

        let mut data = Vec::new();

        for line in reader.lines() {
            data.push(line.unwrap());
        }

        Ok(data)
    }

    /// Parse the string `s`, returning a vector containing the extracted numbers
    /// The numbers in `s` are space-separated
    ///
    /// # Examples
    /// ```
    /// use utils::core::parse_number_list;
    ///
    /// let result: Vec<i32> = parse_number_list("41 48 83 86 17");
    /// assert_eq!(result, [ 41, 48, 83, 86, 17 ]);
    /// let result: Vec<i32> = parse_number_list(" 83 86  6 31 17  9 48 53 ");
    /// assert_eq!(result, [ 83, 86, 6, 31, 17, 9, 48, 53 ]);
    /// ```
    /// Note: the extracted numbers are type-agnostics:
    /// ```
    /// use utils::core::parse_number_list;
    ///
    /// let result: Vec<i32> = parse_number_list("41 48 83 86 17");
    /// assert_eq!(result, [ 41, 48, 83, 86, 17 ]);
    /// let result: Vec<i64> = parse_number_list("41 48 83 86 17");
    /// assert_eq!(result, [ 41, 48, 83, 86, 17 ]);
    /// ```
    pub fn parse_number_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
        parse_number_list_with_separator(s, " ")
    }

    /// Parse the string `s`, using the separator `sep`, returning a vector containing the extracted numbers
    ///
    /// # Examples
    /// ```
    /// use utils::core::parse_number_list_with_separator;
    ///
    /// let result: Vec<i32> = parse_number_list_with_separator("41 48 83 86 17", " ");
    /// assert_eq!(result, [ 41, 48, 83, 86, 17 ]);
    /// let result: Vec<i32> = parse_number_list_with_separator("83 86  6 31 17  9 48 53", " ");
    /// assert_eq!(result, [ 83, 86, 6, 31, 17, 9, 48, 53 ]);
    /// ```
    pub fn parse_number_list_with_separator<T: std::str::FromStr>(s: &str, sep: &str) -> Vec<T> {
        s.split(sep).filter_map(|s| s.parse::<T>().ok()).collect()
    }

    /// Parse the string `s`, returning a vector containing the extracted numbers
    /// The numbers in `s` are comma-separated
    ///
    /// # Examples
    /// ```
    /// use utils::core::parse_comma_number_list;
    ///
    /// let result: Vec<i32> = parse_comma_number_list("41,48,83,86,17");
    /// assert_eq!(result, [ 41, 48, 83, 86, 17 ]);
    /// ```
    /// Note: the extracted numbers are type-agnostics:
    /// ```
    /// use utils::core::parse_comma_number_list;
    ///
    /// let result: Vec<i32> = parse_comma_number_list("41,48,83,86,17");
    /// assert_eq!(result, [ 41, 48, 83, 86, 17 ]);
    /// let result: Vec<i64> = parse_comma_number_list("41,48,83,86,17");
    /// assert_eq!(result, [ 41, 48, 83, 86, 17 ]);
    /// ```
    pub fn parse_comma_number_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
        parse_number_list_with_separator(s, ",")
    }

    /// Parse the string `s`, returning a vector containing the extracted numbers
    /// The numbers in `s` are pipe-separated
    ///
    /// # Examples
    /// ```
    /// use utils::core::parse_pipe_number_list;
    ///
    /// let result: Vec<i32> = parse_pipe_number_list("41|48|83|86|17");
    /// assert_eq!(result, [ 41, 48, 83, 86, 17 ]);
    pub fn parse_pipe_number_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
        parse_number_list_with_separator(s, "|")
    }

    pub fn parse() -> i32 {
        5
    }

}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::core::*;

    #[test]
    fn sample_test() {
        let result: Vec<i32> = parse_number_list("41 48 83 86 17");
        assert_eq!(result, [ 41, 48, 83, 86, 17 ]);
        let result: Vec<i32> = parse_number_list(" 41 48 83 86 17");
        assert_eq!(result, [ 41, 48, 83, 86, 17 ]);
        let result: Vec<i32> = parse_number_list("41 48 83 86 17 ");
        assert_eq!(result, [ 41, 48, 83, 86, 17 ]);
        let result: Vec<i32> = parse_number_list(" 41 48 83 86 17 ");
        assert_eq!(result, [ 41, 48, 83, 86, 17 ]);
        let result: Vec<i32> = parse_number_list(" 83 86  6 31 17  9 48 53 ");
        assert_eq!(result, [ 83, 86, 6, 31, 17, 9, 48, 53 ]);
        let result: Vec<i32> = parse_number_list("83 86  6 31 17  9 48 53 ");
        assert_eq!(result, [ 83, 86, 6, 31, 17, 9, 48, 53 ]);
        let result: Vec<i32> = parse_number_list(" 83 86  6 31 17  9 48 53");
        assert_eq!(result, [ 83, 86, 6, 31, 17, 9, 48, 53 ]);
        let result: Vec<i32> = parse_number_list("83 86  6 31 17  9 48 53");
        assert_eq!(result, [ 83, 86, 6, 31, 17, 9, 48, 53 ]);

        let result: Vec<i32> = parse_number_list("41 48 83 86 17");
        assert_eq!(result, [ 41_i32, 48_i32, 83_i32, 86_i32, 17_i32 ]);
        let result: Vec<i64> = parse_number_list("41 48 83 86 17");
        assert_eq!(result, [ 41_i64, 48_i64, 83_i64, 86_i64, 17_i64 ]);

        let result: Vec<i32> = parse_comma_number_list("41,48,83,86,17");
        assert_eq!(result, [ 41_i32, 48_i32, 83_i32, 86_i32, 17_i32 ]);
        let result: Vec<i64> = parse_comma_number_list("41,48,83,86,17");
        assert_eq!(result, [ 41_i64, 48_i64, 83_i64, 86_i64, 17_i64 ]);

        let result: Vec<i32> = parse_pipe_number_list("41|48|83|86|17");
        assert_eq!(result, [ 41_i32, 48_i32, 83_i32, 86_i32, 17_i32 ]);
        let result: Vec<i64> = parse_pipe_number_list("41|48|83|86|17");
        assert_eq!(result, [ 41_i64, 48_i64, 83_i64, 86_i64, 17_i64 ]);
    }
}
