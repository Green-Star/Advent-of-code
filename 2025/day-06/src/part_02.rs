use std::collections::HashMap;

pub fn resolve(s: &str) -> i64 {
    let transformed_data = transform_data(s);
    let final_result = transformed_data.iter().map(|p| p.solve()).sum();
    final_result
}

fn transform_data(data: &str) -> Vec<Problem> {
    let mut result = HashMap::new();

    let operands = extract_operands(data);
    let operations = extract_operation(data);

    for (i, vector) in operands.iter().enumerate() {
        result.entry(i).and_modify(|p: &mut Problem| p.operands = vector.clone()).or_insert(Problem { operands: vector.clone(), operation: Operation::None });
    }

    for (i, &o) in operations.iter().enumerate() {
        result.entry(i).and_modify(|p| p.operation = o).or_insert(Problem { operands: vec![], operation: o });
    }

    result.into_values().collect()
}

fn extract_operands(data: &str) -> Vec<Vec<i32>> {
    let mut problems_operands = vec![];

    let lines: Vec<&str> = data.lines().collect();
    let lines = &lines[0..lines.len()-1]; // Remove last line: it is the operand line and we won't process it
    // Retrieve all characters in the string and store them in a 2D-char array
    let lines: Vec<Vec<char>> = lines.iter().map(|l| l.chars().map(|c| c).collect()).collect();

    let mut current_problem_operands = vec![];

    // Now, we can iterate over the char array, jumping line by line to gather all chars at 1 given index in the string
    for c in 0..lines[0].len() {
        let mut char_number = vec![];
        for l in 0..lines.len() {
            char_number.push(lines[l][c]);
        }

        // Now we get all chars at a given index, we get the corresponding number
        //  Probably a silly way here: we collect the chars in a string and parse it to get the number
        //  we surely could have done better but nevermind...
        let string_number = char_number.iter().collect::<String>();
        let string_number = string_number.trim();

        // Quick check: We trimmed the string. If it is empty, it means we ran through a column containing only spaces
        //  -> This is a "Problem-separator" column, so we need to end the current problem and start a new one
        if string_number.is_empty() {
            // Note: The problem are RTL but we process the line LTR, so we do need the reverse the current problem numbers before pushing them in the global operands vector
            problems_operands.push(current_problem_operands.into_iter().rev().collect());
            current_problem_operands = vec![];
        // Otherwise, just parse and store the number in the current problem
        } else {
            current_problem_operands.push(string_number.parse().unwrap());
        }
    }
    /* Also, don't forget to add the last problem processed (since we won't get any "space-separator-column" for this one) */
    if !(current_problem_operands.is_empty()) {
        problems_operands.push(current_problem_operands.into_iter().rev().collect());
    }

    problems_operands
}
fn extract_operation(data: &str) -> Vec<Operation> {
    let mut result = vec![];

    let l = data.lines().last().unwrap();
    for o in l.split_whitespace() {
        match o {
            "+" => result.push(Operation::Add),
            "*" => result.push(Operation::Mult),
            _ => panic!("Unknown operation: {o}!"),
        }
    };

    println!("[{:?}]", result);
    result
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn test_part_02() {
        // Encoding issue, I can't write the data in multi-lines :'(
        let test_input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +\n";

        assert_eq!(resolve(test_input), 3263827);
    }


    #[test]
    fn test_transform_data_01() {
        let test_input = "\
123
 45
  6
*
";

        assert_eq!(transform_data(test_input), vec![ Problem { operands: vec![ 356, 24, 1 ], operation: Operation::Mult } ]);
    }

    #[test]
    fn test_transform_data_02() {
        let test_input = "328\n64 \n98 \n+\n";

        assert_eq!(transform_data(test_input), vec![ Problem { operands: vec![ 8, 248, 369 ], operation: Operation::Add } ]);
    }

    #[test]
    fn test_transform_data_03() {
        let test_input =
" 51\n\
387\n\
215\n\
*\n";

        assert_eq!(transform_data(test_input), vec![ Problem { operands: vec![ 175, 581, 32 ], operation: Operation::Mult } ]);
    }

    #[test]
    fn test_transform_data_04() {
        let test_input = "\
64 \n\
23 \n\
314\n\
+\n\
";

        assert_eq!(transform_data(test_input), vec![ Problem { operands: vec![ 4, 431, 623 ], operation: Operation::Add } ]);
    }


    #[test]
    fn test_solve_problem_01() {
        let test = Problem { operands: vec![ 4, 431, 623 ], operation: Operation::Add };
        assert_eq!(test.solve(), 1058);
    }

    #[test]
    fn test_solve_problem_02() {
        let test = Problem { operands: vec![ 175, 581, 32 ], operation: Operation::Mult };
        assert_eq!(test.solve(), 3253600);
    }

    #[test]
    fn test_solve_problem_03() {
        let test = Problem { operands: vec![ 8, 248, 369 ], operation: Operation::Add };
        assert_eq!(test.solve(), 625);
    }

    #[test]
    fn test_solve_problem_04() {
        let test = Problem { operands: vec![ 356, 24, 1 ], operation: Operation::Mult };
        assert_eq!(test.solve(), 8544);
    }
}
