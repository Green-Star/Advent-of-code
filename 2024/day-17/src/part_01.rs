use crate::core::parse_number_list;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let machine = transform_data(data);

  println!("{:?}", machine);
  let finished_machine = machine.process_until_halt();
  println!("{:?}", finished_machine);


  let final_result = 0;
  println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> StateMachine {
  let s = data[0].split(": ");
  let a = s.last().unwrap().parse().unwrap();

  let s = data[1].split(": ");
  let b = s.last().unwrap().parse().unwrap();

  let s = data[2].split(": ");
  let c = s.last().unwrap().parse().unwrap();

  let s = data[4].split(": ");
  let opcodes = parse_number_list(s.last().unwrap());

  StateMachine { a, b, c, instruction: 0, opcodes, outputs: vec![] }
}

#[derive(Debug, Clone)]
struct StateMachine {
  a: i32,
  b: i32,
  c: i32,

  instruction: usize,
  opcodes: Vec<i32>,

  outputs: Vec<i32>,
}
impl StateMachine {
  fn process_until_halt(&self) -> StateMachine {
    let mut result = self.clone();

    loop {
      match result.opcodes.get(result.instruction) {
        Some(op) => {
          let instruction = *op;
          let operand = result.opcodes[result.instruction + 1];
          result.instruction += 2;
          result = result.process_next_instruction(instruction, operand);
        },
        None => break
      }
    }

    result
  }

  fn process_next_instruction(&self, instruction: i32, operand: i32) -> StateMachine {
    let mut out = self.outputs.clone();
    out.push(operand);
    StateMachine { a: self.a, b: self.b, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: out }
  }

  fn get_output_string(&self) -> String {
    let mut s = "".to_string();

    for i in &self.outputs {
      s = format!("{s},{i}");
    }

    s.strip_prefix(",").unwrap().to_string()
  }
}
