use std::collections::HashMap;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let (patterns, designs) = transform_data(data);
  let possible_designs = try_building_all_designs(&designs, &patterns);

  let final_result = possible_designs.len();
  println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> (Vec<Pattern>, Vec<Design>) {
  let patterns: Vec<String> = data[0].split(",").map(|s| s.trim().to_string()).collect();
  let designs: Vec<String> = data.iter().skip(2).map(|s| s.to_string()).collect();

  (patterns, designs)
}


#[derive(Debug, Clone)]
enum OpCode {
  AND,
  OR,
  XOR,
}
impl OpCode {
  fn process(&self, x: i64, y: i64) -> i64 {
    match self {
      &Self::AND => x & y,
      &Self::OR => x | y,
      &Self::XOR => x ^ y,
    }
  }
}


#[derive(Debug, Clone)]
struct Gate {
  a: String,
  b: String,
  operation: OpCode,
  output: String,

  processed: bool,
  value: i64,
}

type Register = HashMap<String, Option<i64>>;

#[derive(Debug, Clone)]
struct Device {
  registers: Register,
  gates: Vec<Gate>,
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
  fn get_combo_operand_value(&self, operand: i32) -> i32 {
    match operand {
      0..=3 => operand,
      4 => self.a,
      5 => self.b,
      6 => self.c,
      _ => panic!("Not a valid operand!"),
    }
  }
  fn do_dv(&self, operand: i32) -> i32 {
    let denominator = self.get_combo_operand_value(operand);
    self.a >> denominator
  }
  fn adv(&self, operand: i32) -> StateMachine {
    let result = self.do_dv(operand);
    StateMachine { a: result, b: self.b, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn bxl(&self, operand: i32) -> StateMachine {
    let result = self.b ^ operand;
    StateMachine { a: self.a, b: result, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn bst(&self, operand: i32) -> StateMachine {
    let result = self.get_combo_operand_value(operand) & 7;
    StateMachine { a: self.a, b: result, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn jnz(&self, operand: i32) -> StateMachine {
    if self.a == 0 {
      StateMachine { a: self.a, b: self.b, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
    } else {
      StateMachine { a: self.a, b: self.b, c: self.c, instruction: operand as usize, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
    }
  }
  fn bxc(&self, _: i32) -> StateMachine {
    let result = self.b ^ self.c;
    StateMachine { a: self.a, b: result, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn out(&self, operand: i32) -> StateMachine {
    let mut result = self.outputs.clone();
    result.push(self.get_combo_operand_value(operand) & 7);
    StateMachine { a: self.a, b: self.b, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: result }
  }
  fn bdv(&self, operand: i32) -> StateMachine {
    let result = self.do_dv(operand);
    StateMachine { a: self.a, b: result, c: self.c, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }
  fn cdv(&self, operand: i32) -> StateMachine {
    let result = self.do_dv(operand);
    StateMachine { a: self.a, b: self.b, c: result, instruction: self.instruction, opcodes: self.opcodes.clone(), outputs: self.outputs.clone() }
  }

  fn get_output_string(&self) -> String {
    let mut s = "".to_string();

    for i in &self.outputs {
      s = format!("{s},{i}");
    }

    s.strip_prefix(",").unwrap().to_string()
  }
}



type Pattern = String;
type Design = String;

fn try_building_all_designs(designs: &Vec<Design>, patterns: &Vec<Pattern>) -> Vec<()> {
  designs.iter().filter_map(|d| try_building_design(d, patterns)).collect()
}

fn try_building_design(design: &Design, patterns: &Vec<Pattern>) -> Option<()> {
  if design.is_empty() { return Some(()) }

  for p in patterns {
    if design.starts_with(p) {
      let is_possible = try_building_design(&String::from_iter(design.chars().skip(p.len())), patterns);
      if let Some(_) = is_possible { return is_possible }
    }
  }

  None
}
