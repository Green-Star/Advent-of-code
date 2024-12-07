pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let transformed_data = transform_data(data);

  let mut computed_calibrations = Vec::new();
  for mut c in transformed_data {
    c.init();
    c.compute();
    computed_calibrations.push(c);
  }

  let final_result: u64 = computed_calibrations.iter().filter(|c| c.found).map(|c| c.final_result).sum();

  println!("Part 1 final result: {:?}", final_result);
}

fn transform_data(data: Vec<String>) -> Vec<Calibration> {
  let mut result = Vec::new();

  for line in data {
    let mut splitted = line.split(":");

    let final_result = splitted.next().unwrap().parse().unwrap();
    let number_list = crate::core::parse_number_list::<u64>(splitted.next().unwrap());

    result.push(Calibration { final_result, operands: number_list, partial_results: vec![], found: false });
  }

  result
}


#[derive(Debug, Clone)]
struct Calibration {
  final_result: u64,

  operands: Vec<u64>,
  partial_results: Vec<u64>,

  found: bool
}
impl Calibration {
  fn init(&mut self) {
    self.partial_results.push(self.operands.remove(0));
  }

  fn compute(&mut self) {
    while self.operands.is_empty() == false {
      if self.partial_results.is_empty() { return }

      let mut result = Vec::new();
      let y = self.operands.remove(0);

      for x in &(self.partial_results) {
        let (plus, mult) = (x + y, x * y);
        if plus <= self.final_result { result.push(plus); }
        if mult <= self.final_result { result.push(mult); }
      }

      self.partial_results = result;
    }
    self.found = self.partial_results.iter().any(|x| *x == self.final_result);
  }
}
