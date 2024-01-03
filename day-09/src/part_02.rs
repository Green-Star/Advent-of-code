fn transform_data(data: Vec<String>) -> Vec<Oasis> {
    let mut transformed = Vec::new();

    for s in data {
        transformed.push(Oasis::new(crate::core::parse_number_list(&s)));
    }

    transformed
}

struct Oasis {
    input: Vec<i64>,

    history: Vec<Vec<i64>>,
}
impl Oasis {
    fn new(input: Vec<i64>) -> Oasis {
        Oasis { input, history: Vec::new() }
    }

    fn compute(mut self) -> Result<Self, ()> {
        let history = self.compute_next_line(&self.input);

        self.history = vec![history];

        loop {
            if self.is_finished() { break }

            let next_line = self.compute_next_line(&self.history.last().unwrap());
            self.history.push(next_line);
        }

        Ok(self)
    }

    fn is_finished(&self) -> bool {
        let last = self.history.last().unwrap();
        for i in last {
            if i != &0 { return false }
        }
        true
    }

    fn compute_next_line(&self, last_line: &Vec<i64>) -> Vec<i64> {
        let mut first = last_line.iter();
        let mut second = last_line.iter();

        let mut next_line = Vec::new();
        second.next();

        for b in second {
            let a = first.next().unwrap();

            next_line.push(b - a);
        }

        next_line
    }

    fn compute_final_result(&self) -> i64 {
        self.input.first().unwrap() - self.history.iter().rev().fold(0, |result, h| h.first().unwrap() - result)
    }
}

pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let transformed_data = transform_data(data);
    let computed_data: Vec<Oasis> = transformed_data.into_iter().map(|oasis| oasis.compute().unwrap()).collect();

    let final_result = computed_data.iter().fold(0, |result, oasis| result + oasis.compute_final_result());

    println!("Part 2 final result: {}", final_result);
}
