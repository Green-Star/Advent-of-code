use std::result;

use crate::core::parse_number_list;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let machine = transform_data(data);

  println!("{:?}", machine);

  let mut start: i64 = 35282534841844;
// -> too high! let mut start: i64 = 106094283126537;
// between
//    106094283126537
//     13261785268224
  /* Borrowing issue... */
  let target= machine.opcodes.clone();

  let start = find_output_pattern(&machine.opcodes, target);
  println!("Seed found: {}", start);
  let mut test = machine.clone();
  test.a = start;
  test = test.process_until_halt().unwrap();
  println!("{:?}", test);


  let final_result = 0;
  println!("Part 2 final result: {}", final_result);
}

fn find_output_pattern(opcodes: &Vec<i64>, target: Vec<i64>) -> i64 {
  let mut a_start = if target.len() == 1 {
    0
  } else {
    8 * find_output_pattern(opcodes, target[1..].to_vec())
  };

  loop {
    let machine = StateMachine { a: a_start, b: 0, c: 0, instruction: 0, opcodes: opcodes.clone(), outputs: vec![] };
    let out = machine.process_until_halt().unwrap();

    if out.outputs == target { break }
    a_start += 1;
  }

  println!("Found length {} - {} / Pattern {:?}", target.len(), a_start, target);

  a_start
}

/*
fn findAMatchingOutput(program: Vec<i32>, target: Vec<i32>) -> i32 {
  let mut aStart = if (target.len() == 1) {
      0
  } else {
      8 * findAMatchingOutput(program, &target[1..])
  }

  while( run(program, aStart) != target) {
      aStart++
  }

  return aStart
}
  */

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
  a: i64,
  b: i64,
  c: i64,

  instruction: usize,
  opcodes: Vec<i64>,

  outputs: Vec<i64>,
}
impl StateMachine {
  fn process_until_halt(&self) -> Option<StateMachine> {
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

    Some(result)
  }

  fn process_next_instruction(&self, instruction: i64, operand: i64) -> StateMachine {
    match instruction {
      0 => self.adv(operand),
      1 => self.bxl(operand),
      2 => self.bst(operand),
      3 => self.jnz(operand),
      4 => self.bxc(operand),
      5 => self.out(operand),
      6 => self.bdv(operand),
      7 => self.cdv(operand),
      _ => StateMachine { a: self.a, b: self.b, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
    }
  }
  fn get_combo_operand_value(&self, operand: i64) -> i64 {
    match operand {
      0..=3 => operand,
      4 => self.a,
      5 => self.b,
      6 => self.c,
      _ => panic!("Not a valid operand!"),
    }
  }
  fn do_dv(&self, operand: i64) -> i64 {
    let denominator = self.get_combo_operand_value(operand);
    self.a >> denominator
  }
  fn adv(&self, operand: i64) -> StateMachine {
    let result = self.do_dv(operand);
    StateMachine { a: result, b: self.b, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn bxl(&self, operand: i64) -> StateMachine {
    let result = self.b ^ operand;
    StateMachine { a: self.a, b: result, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn bst(&self, operand: i64) -> StateMachine {
    let result = self.get_combo_operand_value(operand) & 7;
    StateMachine { a: self.a, b: result, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn jnz(&self, operand: i64) -> StateMachine {
    if self.a == 0 {
      StateMachine { a: self.a, b: self.b, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
    } else {
      StateMachine { a: self.a, b: self.b, c: self.c, instruction: operand as usize, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
    }
  }
  fn bxc(&self, _: i64) -> StateMachine {
    let result = self.b ^ self.c;
    StateMachine { a: self.a, b: result, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn out(&self, operand: i64) -> StateMachine {
    let mut result = self.outputs.clone();
    result.push(self.get_combo_operand_value(operand) & 7);
    StateMachine { a: self.a, b: self.b, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: result }
  }
  fn bdv(&self, operand: i64) -> StateMachine {
    let result = self.do_dv(operand);
    StateMachine { a: self.a, b: result, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn cdv(&self, operand: i64) -> StateMachine {
    let result = self.do_dv(operand);
    StateMachine { a: self.a, b: self.b, c: result, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }


  fn test(&self, instruction: i64, operand: i64) -> StateMachine {
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
