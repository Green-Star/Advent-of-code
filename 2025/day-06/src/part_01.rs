use std::collections::HashMap;

pub fn resolve(s: &str) -> i64 {
    let transformed_data = transform_data(s);
    let final_result = transformed_data.iter().map(|p| p.solve()).sum();
    final_result
}

fn transform_data(data: &str) -> Vec<Problem> {
    let mut result = HashMap::new();

    for l in data.lines() {
        for (i, o) in l.split_whitespace().enumerate() {
            match o {
                "+" => result.entry(i).and_modify(|p: &mut Problem| p.operation = Operation::Add).or_insert(Problem { operands: vec![], operation: Operation::Add }),
                "*" => result.entry(i).and_modify(|p| p.operation = Operation::Mult).or_insert(Problem { operands: vec![], operation: Operation::Mult }),
                _ => result.entry(i).and_modify(|p| p.operands.push(o.parse().unwrap())).or_insert(Problem { operands: vec![ o.parse().unwrap() ], operation: Operation::None }),
            };
        }
    }

    result.into_values().collect()
}


#[derive(Debug, PartialEq, Eq)]
enum Operation {
    None,
    Add,
    Mult,
}

#[derive(Debug, PartialEq, Eq)]
struct Problem {
    operands: Vec<i32>,
    operation: Operation,
}
impl Problem {
    fn solve(&self) -> i64 {
        match self.operation {
            Operation::Add => self.operands.iter().fold(0, |acc, &x| acc + x as i64),
            Operation::Mult => self.operands.iter().fold(1, |acc, &x| acc * x as i64),
            Operation::None => panic!("No operation in {:?}!", self),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let test_input = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

        assert_eq!(resolve(test_input), 4277556);
    }


    #[test]
    fn test_transform_data_01() {
        let test_input = "\
123
 45
  6
*
";

        assert_eq!(transform_data(test_input), vec![ Problem { operands: vec![ 123, 45, 6 ], operation: Operation::Mult } ]);
    }

    #[test]
    fn test_transform_data_02() {
        let test_input = "\
        328
        64
        98
        +
        ";

        assert_eq!(transform_data(test_input), vec![ Problem { operands: vec![ 328, 64, 98 ], operation: Operation::Add } ]);
    }

    #[test]
    fn test_transform_data_03() {
        let test_input = "\
        51
        387
        215
        *
        ";

        assert_eq!(transform_data(test_input), vec![ Problem { operands: vec![ 51, 387, 215 ], operation: Operation::Mult } ]);
    }

    #[test]
    fn test_transform_data_04() {
        let test_input = "\
        64
        23
        314
        +
        ";

        assert_eq!(transform_data(test_input), vec![ Problem { operands: vec![ 64, 23, 314 ], operation: Operation::Add } ]);
    }


    #[test]
    fn test_solve_problem_01() {
        let test = Problem { operands: vec![ 123, 45, 6 ], operation: Operation::Mult };
        assert_eq!(test.solve(), 33210);
    }

    #[test]
    fn test_solve_problem_02() {
        let test = Problem { operands: vec![ 328, 64, 98 ], operation: Operation::Add };
        assert_eq!(test.solve(), 490);
    }

    #[test]
    fn test_solve_problem_03() {
        let test = Problem { operands: vec![ 51, 387, 215 ], operation: Operation::Mult };
        assert_eq!(test.solve(), 4243455);
    }

    #[test]
    fn test_solve_problem_04() {
        let test = Problem { operands: vec![ 64, 23, 314 ], operation: Operation::Add };
        assert_eq!(test.solve(), 401);
    }
}
