#[derive(Debug, PartialEq, Clone, Copy)]
enum RockType {
    Empty,
    Round,
    Cube,
}

#[derive(Debug, PartialEq)]
struct Reflector {
    rocks: Vec<Vec<RockType>>,
}
impl Reflector {
    fn slide_north(&mut self) {
        for j in 0..self.rocks[0].len() {
            self.slide_one_column_north(j);
        }
    }

    fn slide_one_column_north(&mut self, column: usize) {
        for i in 0..self.rocks.len() {
            match self.rocks[i][column] {
                RockType::Round => self.slide_one_rock_north(i, column),
                _ => {},
            }
        }
    }

    fn slide_one_rock_north(&mut self, mut line: usize, column: usize) {
        loop {
            if line == 0 { break; }

            match self.rocks[line - 1][column] {
                RockType::Empty => {
                    self.rocks[line - 1][column] = self.rocks[line][column];
                    self.rocks[line][column] = RockType::Empty;
                },
                _ => break,
            }

            line = line - 1;
        }
    }

    fn get_one_column_value(&self, column: usize) -> i32 {
        let length = self.rocks[column].len();
        let mut result = 0;
        for i in 0..length {
            match self.rocks[i][column] {
                RockType::Round => result += length - i,
                _ => {},
            }
        }
        result as i32
    }

    fn get_final_result(&self) -> i32 {
        self.rocks.iter().enumerate().fold(0, |acc, (index, _)| acc + self.get_one_column_value(index))
    }
}

pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let mut transformed_data = transform_data(data);
    transformed_data.slide_north();
    let final_result = transformed_data.get_final_result();

    println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> Reflector {
    let mut rocks = Vec::new();

    for line in data {
        let mut rock_line = Vec::new();
        for c in line.chars() {
            let rock =
                match c {
                    'O' => RockType::Round,
                    '#' => RockType::Cube,
                    _ => RockType::Empty,
                };
            rock_line.push(rock);
        }
        rocks.push(rock_line);
    }

    Reflector { rocks }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_load_and_transform_data() {
        let test_data = Reflector { rocks: vec![
            vec![RockType::Round, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty],
            vec![RockType::Round, RockType::Empty, RockType::Round, RockType::Round, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube],
            vec![RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty],
            vec![RockType::Round, RockType::Round, RockType::Empty, RockType::Cube, RockType::Round, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Round],
            vec![RockType::Empty, RockType::Round, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Round, RockType::Cube, RockType::Empty],
            vec![RockType::Round, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Round, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Cube],
            vec![RockType::Empty, RockType::Empty, RockType::Round, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Round, RockType::Empty, RockType::Empty, RockType::Round],
            vec![RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Round, RockType::Empty, RockType::Empty],
            vec![RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Cube, RockType::Cube, RockType::Empty, RockType::Empty],
            vec![RockType::Cube, RockType::Round, RockType::Round, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty],
        ] };

        let data = crate::core::load_file_in_memory("./test.data").unwrap();
        let result = transform_data(data);

        assert_eq!(test_data, result);
    }

    #[test]
    fn it_should_slide_one_column_north() {
        let test_data = Reflector { rocks : vec![
            vec![RockType::Round, RockType::Round, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Round, RockType::Empty, RockType::Empty],
            vec![RockType::Round, RockType::Round, RockType::Round, RockType::Round, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube],
            vec![RockType::Round, RockType::Round, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty],
            vec![RockType::Round, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Round, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Round],
            vec![RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty],
            vec![RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Round, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Cube],
            vec![RockType::Empty, RockType::Empty, RockType::Round, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Round, RockType::Round, RockType::Empty, RockType::Round],
            vec![RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty],
            vec![RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Cube, RockType::Cube, RockType::Empty, RockType::Empty],
            vec![RockType::Cube, RockType::Empty, RockType::Round, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty],
        ] };

        let data = crate::core::load_file_in_memory("./test.data").unwrap();
        let mut test = transform_data(data);

        test.slide_one_column_north(0);
        test.slide_one_column_north(1);
        test.slide_one_column_north(7);
        test.slide_one_column_north(8);

        assert_eq!(test, test_data);
    }

    #[test]
    fn it_should_slide_all_columns_north() {
        let test_data = Reflector { rocks : vec![
            vec![RockType::Round, RockType::Round, RockType::Round, RockType::Round, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Round, RockType::Empty, RockType::Empty],
            vec![RockType::Round, RockType::Round, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube],
            vec![RockType::Round, RockType::Round, RockType::Empty, RockType::Empty, RockType::Round, RockType::Cube, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Round],
            vec![RockType::Round, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Round, RockType::Round, RockType::Empty, RockType::Empty, RockType::Empty],
            vec![RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty],
            vec![RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Cube],
            vec![RockType::Empty, RockType::Empty, RockType::Round, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Round, RockType::Empty, RockType::Round],
            vec![RockType::Empty, RockType::Empty, RockType::Round, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty],
            vec![RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Cube, RockType::Cube, RockType::Empty, RockType::Empty],
            vec![RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty],
        ] };

        let data = crate::core::load_file_in_memory("./test.data").unwrap();
        let mut test = transform_data(data);

        test.slide_north();

        assert_eq!(test, test_data);
    }

    #[test]
    fn it_should_get_one_column_value() {
        let test_data = Reflector { rocks : vec![
            vec![RockType::Round, RockType::Round, RockType::Round, RockType::Round, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Round, RockType::Empty, RockType::Empty],
            vec![RockType::Round, RockType::Round, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube],
            vec![RockType::Round, RockType::Round, RockType::Empty, RockType::Empty, RockType::Round, RockType::Cube, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Round],
            vec![RockType::Round, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Round, RockType::Round, RockType::Empty, RockType::Empty, RockType::Empty],
            vec![RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty],
            vec![RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Cube],
            vec![RockType::Empty, RockType::Empty, RockType::Round, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Round, RockType::Empty, RockType::Round],
            vec![RockType::Empty, RockType::Empty, RockType::Round, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty],
            vec![RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Cube, RockType::Cube, RockType::Empty, RockType::Empty],
            vec![RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Cube, RockType::Empty, RockType::Empty, RockType::Empty, RockType::Empty],
        ] };

        assert_eq!(test_data.get_one_column_value(0), 10+9+8+7);
        assert_eq!(test_data.get_one_column_value(1), 10+9+8);
        assert_eq!(test_data.get_one_column_value(2), 10+4+3);
        assert_eq!(test_data.get_one_column_value(3), 10);
        assert_eq!(test_data.get_one_column_value(4), 8);
        assert_eq!(test_data.get_one_column_value(5), 7);
        assert_eq!(test_data.get_one_column_value(6), 7);
        assert_eq!(test_data.get_one_column_value(7), 10+4);
        assert_eq!(test_data.get_one_column_value(8), 0);
        assert_eq!(test_data.get_one_column_value(9), 8+4);
    }

    #[test]
    fn it_should_get_the_final_result() {
        let test_data = 136;

        let data = crate::core::load_file_in_memory("./test.data").unwrap();
        let mut test = transform_data(data);
        test.slide_north();

        assert_eq!(test.get_final_result(), test_data);
    }
}
