use std::{iter, ops::Add, time::Instant};

use z3::{Context, Optimize, SatResult, Solver, ast::{Bool, Int}};

pub fn resolve(s: &str) -> usize {
    let transformed_data = transform_data(s);
    println!("1: {:?}", transformed_data[0]);
    println!("2: {:?}", transformed_data[1]);
    println!("3: {:?}", transformed_data[2]);

    solver();

    let final_result = 0;
    final_result
}

fn solver() {
    let solver = Solver::new();

    let x = Int::new_const("x");
    let y = Int::new_const("y");

    solver.assert(&x.ne(0));
    solver.assert(&y.ne(0));

    let area = &x * &x + &y * &y;
    solver.assert(area.eq(25));

   	println!("{solver:?}");

    for (x,y) in solver.solutions((x, y), true) {
        println!("{} / {}", x.as_i64().unwrap(), y.as_i64().unwrap());
    }

    println!("New example");

    let optimizer = Optimize::new();

    let c1 = Int::new_const("c1");
    let c5 = Int::new_const("c5");
    let c10 = Int::new_const("c10");

    let total = (&c1 *1) + (&c5 * 5) + (&c10 * 10);
    let count = &c1 + &c5 + &c10;

    optimizer.assert(&c1.ge(0));
    optimizer.assert(&c5.ge(0));
    optimizer.assert(&c10.ge(0));

    optimizer.assert(&total.eq(37));
    optimizer.minimize(&count);

    if let z3::SatResult::Sat = optimizer.check(&[]) {
        let model = optimizer.get_model().unwrap();
        let c1 = model.eval(&c1, true).unwrap();
        let c5 = model.eval(&c5, true).unwrap();
        let c10 = model.eval(&c10, true).unwrap();

        println!("Solution: {c1}*1, {c5}*5, {c10}*10");
    } else {
        println!("Unsat");
    }


    println!("Nouvel exemple non général");
    let problem = Problem::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
    println!("----");
    println!("Pour info: {:?}", problem);

    let optimizer = Optimize::new();

//    let target: Vec<Bool> = problem.target.iter().enumerate().map(|(i, b)| Bool::new_const(format!("target_{i}"))).collect();

    let buttons: Vec<Bool> = problem.buttons.iter().enumerate().map(|(i, _)| Bool::new_const(format!("button_{i}"))).collect();
    (0..problem.target.len()).for_each(|i| {
        let triggers: Vec<_> = problem.buttons
                                            .iter()
                                            .enumerate()
                                            .filter(|(_, b)| b.contains(&i))
                                            .map(|(i, _)| &buttons[i])
                                            .collect();
        let sum = triggers.iter().fold(Int::from_i64(0), |acc, b| {
            acc.add(&b.ite(&Int::from_i64(1), &Int::from_i64(0)))
        });

        let activated = sum.modulo(2).eq(1);

        optimizer.assert(&activated.eq(Bool::from_bool(problem.target[i])));
    });

    let presses = Int::add(&buttons.iter().map(|b| b.ite(&Int::from_i64(1), &Int::from_i64(0))).collect::<Vec<_>>());
    optimizer.minimize(&presses);

    println!("{:?}", optimizer);
    match optimizer.check(&[]) {
        SatResult::Sat => {
            println!("Trouvé !!!");
            let model = optimizer.get_model().unwrap();
            let result = model.eval(&presses, true).unwrap();

            println!("Minimal presses found: {result}");
        },
        _ => { println!("No solution"); }
    }



    println!("----");
    println!("2nd example");
    let problem = Problem::from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");

    let optimizer = Optimize::new();

    let buttons: Vec<Bool> = problem.buttons.iter().enumerate().map(|(i, _)| Bool::new_const(format!("button_{i}"))).collect();
    (0..problem.target.len()).for_each(|i| {
        let triggers: Vec<_> = problem.buttons
                                            .iter()
                                            .enumerate()
                                            .filter(|(_, b)| b.contains(&i))
                                            .map(|(i, _)| &buttons[i])
                                            .collect();
        let sum = triggers.iter().fold(Int::from_i64(0), |acc, b| {
            acc.add(&b.ite(&Int::from_i64(1), &Int::from_i64(0)))
        });

        let activated = sum.modulo(2).eq(1);

        optimizer.assert(&activated.eq(Bool::from_bool(problem.target[i])));
    });

    let presses = Int::add(&buttons.iter().map(|b| b.ite(&Int::from_i64(1), &Int::from_i64(0))).collect::<Vec<_>>());
    optimizer.minimize(&presses);

    match optimizer.check(&[]) {
        SatResult::Sat => {
            let model = optimizer.get_model().unwrap();
            let result = model.eval(&presses, true).unwrap();

            println!("Minimal presses found: {result}");
        },
        _ => { println!("No solution"); }
    }




    println!("----");
    println!("Third example");
    let problem = Problem::from("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");

    let optimizer = Optimize::new();

    let buttons: Vec<Bool> = problem.buttons.iter().enumerate().map(|(i, _)| Bool::new_const(format!("button_{i}"))).collect();
    (0..problem.target.len()).for_each(|i| {
        let triggers: Vec<_> = problem.buttons
                                            .iter()
                                            .enumerate()
                                            .filter(|(_, b)| b.contains(&i))
                                            .map(|(i, _)| &buttons[i])
                                            .collect();
        let sum = triggers.iter().fold(Int::from_i64(0), |acc, b| {
            acc.add(&b.ite(&Int::from_i64(1), &Int::from_i64(0)))
        });

        let activated = sum.modulo(2).eq(1);

        optimizer.assert(&activated.eq(Bool::from_bool(problem.target[i])));
    });

    let presses = Int::add(&buttons.iter().map(|b| b.ite(&Int::from_i64(1), &Int::from_i64(0))).collect::<Vec<_>>());
    optimizer.minimize(&presses);

    match optimizer.check(&[]) {
        SatResult::Sat => {
            let model = optimizer.get_model().unwrap();
            let result = model.eval(&presses, true).unwrap();

            println!("Minimal presses found: {result}");
        },
        _ => { println!("No solution"); }
    }

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
