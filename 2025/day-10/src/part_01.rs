use std::ops::Add;

use z3::{Optimize, SatResult, ast::{Bool, Int}};

pub fn resolve(s: &str) -> usize {
    let transformed_data = transform_data(s);


println!("Et maintenant en dheors: ");
    println!("21: {:?}", transformed_data[0].solve());
    println!("21: {:?}", transformed_data[1].solve());
    println!("21: {:?}", transformed_data[2].solve());

    let final_result = 0;
    final_result
}

fn transform_data(data: &str) -> Vec<Problem> {
    data.lines().map(|s| Problem::from(s)).collect()
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Problem {
    target: Vec<bool>,

    buttons: Vec<Vec<usize>>,
    joltage: Vec<i32>,
}
impl Problem {
    fn from(s: &str) -> Self {
        let mut splitted = s.split_whitespace();

        // Target is always the first part of the string
        let target_string = splitted.next().unwrap();

        // Then, iterate over the string until we reach the voltage part
        let joltage_string;
        let mut buttons_string = vec![];
        loop {
            let ss = splitted.next().unwrap();
            if ss.starts_with("{") {
                joltage_string = ss;
                break;
            }
            buttons_string.push(ss);
        }

        // And now, time to parse everything
        let target = Self::parse_target(target_string);
        let buttons = Self::parse_buttons(buttons_string);
        let joltage = Self::parse_joltage(joltage_string);

        Problem { target, buttons, joltage }
    }

    fn parse_target(s: &str) -> Vec<bool> {
        let sanitized = s.replace(&['[',']'][..], "");
        let mut target = vec![];
        for c in sanitized.chars() {
            if c == '#' {
                target.push(true);
            } else {
                target.push(false);
            }
        }
        target
    }
    fn parse_buttons(v: Vec<&str>) -> Vec<Vec<usize>> {
        let sanitized: Vec<String> = v.iter().map(|&s| s.replace(&['(',')'][..], "")).collect();
        sanitized.iter().map(|s| utils::core::parse_comma_number_list(s)).collect()
    }
    fn parse_joltage(s: &str) -> Vec<i32> {
        let sanitized = s.replace(&['{','}'][..], "");
        utils::core::parse_comma_number_list(&sanitized)
    }

    fn solve(&self) -> Option<i64> {
        let optimizer = Optimize::new();

        // Create Z3 variables (one for each button to press)
        let buttons: Vec<Bool> = self.buttons.iter().enumerate().map(|(i, _)| Bool::new_const(format!("button_{i}"))).collect();
        // For each target value
        //  retrieve all the buttons referencing it
        //  sum the number of times these buttons get pressed to determine the value of the boolean (for this target)
        //  assert the boolean has to be equal to the corresponding target boolean
        for (index, target) in self.target.iter().enumerate() {
            let triggers: Vec<_> = self.buttons
                                            .iter()
                                            .enumerate()
                                            .filter(|(_, b)| b.contains(&index))
                                            .map(|(i, _)| &buttons[i])
                                            .collect();
            let sum = triggers.iter().fold(Int::from_i64(0), |acc, b| {
                acc.add(&b.ite(&Int::from_i64(1), &Int::from_i64(0)))
            });

            let activated = sum.modulo(2).eq(1);

            optimizer.assert(&activated.eq(Bool::from_bool(*target)));
        }

        // Get the number of total presses on the button and minimize it
        let presses = Int::add(&buttons.iter().map(|b| b.ite(&Int::from_i64(1), &Int::from_i64(0))).collect::<Vec<_>>());
        optimizer.minimize(&presses);

        match optimizer.check(&[]) {
            SatResult::Sat => {
                let model = optimizer.get_model().unwrap();
                let result = model.eval(&presses, true).unwrap();

                Some(result.as_i64().unwrap())
            },
            _ => { None }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let test_input = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

        assert_eq!(resolve(test_input), 7);
    }

    #[test]
    fn test_first_input() {
        let test_input = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
";

        assert_eq!(resolve(test_input), 7);
    }
    #[test]
    fn test_second_input() {
        let test_input = "\
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
";

        assert_eq!(resolve(test_input), 7);
    }
    #[test]
    fn test_third_input() {
        let test_input = "\
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

        assert_eq!(resolve(test_input), 7);
    }
}
