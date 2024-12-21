use crate::core::parse_number_list;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let machine = transform_data(data);

  println!("{:?}", machine);
  let finished_machine = machine.process_until_halt();
  println!("{:?}", finished_machine);

  let final_result = finished_machine.get_output_string();
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
    match instruction {
      0 => self.adv(instruction, operand),
      1 => self.bxl(instruction, operand),
      2 => self.bst(instruction, operand),
      3 => self.jnz(instruction, operand),
      4 => self.bxc(instruction, operand),
      5 => self.out(instruction, operand),
      6 => self.bdv(instruction, operand),
      7 => self.cdv(instruction, operand),
      _ => StateMachine { a: self.a, b: self.b, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
    }
  }
  fn get_combo_operand_value(&self, operand: i32) -> i32 {
    match operand {
      0..=3 => operand,
      4 => self.a,
      5 => self.b,
      6 => self.c,
      _ => panic!("Not a valid operand!"),
    }
  }
  fn do_dv(&self, instruction: i32, operand: i32) -> i32 {
    let denominator = self.get_combo_operand_value(operand);
    self.a >> denominator
  }
  fn adv(&self, instruction: i32, operand: i32) -> StateMachine {
    let result = self.do_dv(instruction, operand);
    StateMachine { a: result, b: self.b, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn bxl(&self, instruction: i32, operand: i32) -> StateMachine {
    let result = self.b ^ operand;
    StateMachine { a: self.a, b: result, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn bst(&self, instruction: i32, operand: i32) -> StateMachine {
    let result = self.get_combo_operand_value(operand) & 7;
    StateMachine { a: self.a, b: result, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn jnz(&self, instruction: i32, operand: i32) -> StateMachine {
    if self.a == 0 {
      StateMachine { a: self.a, b: self.b, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
    } else {
      StateMachine { a: self.a, b: self.b, c: self.c, instruction: operand as usize, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
    }
  }
  fn bxc(&self, instruction: i32, operand: i32) -> StateMachine {
    let result = self.b ^ self.c;
    StateMachine { a: self.a, b: result, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn out(&self, instruction: i32, operand: i32) -> StateMachine {
    let mut result = self.outputs.clone();
    result.push(self.get_combo_operand_value(operand) & 7);
    StateMachine { a: self.a, b: self.b, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: result }
  }
  fn bdv(&self, instruction: i32, operand: i32) -> StateMachine {
    let result = self.do_dv(instruction, operand);
    StateMachine { a: self.a, b: result, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn cdv(&self, instruction: i32, operand: i32) -> StateMachine {
    let result = self.do_dv(instruction, operand);
    StateMachine { a: self.a, b: self.b, c: result, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }


  fn test(&self, instruction: i32, operand: i32) -> StateMachine {
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
