use std::{collections::HashMap, result};

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let mut device = transform_data(data);

  println!("{:?}", device);

  let final_result = 0;
  println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> Device {
  let mut registers = HashMap::new();
  let mut gates = vec![];

  for line in data {
    let mut s = line.split(": ");
    if let Some(register) = s.next() {
      let value: i64 = s.last().unwrap().parse().unwrap();
      registers.insert(register.to_string(), Some(value));
    }

    let mut s = line.split(" -> ");
    if let Some(gate) = s.next() {
      let mut subsplit = gate.split(" ");

      let a = subsplit.next().unwrap();
      let opcode = match subsplit.next().unwrap() {
        "AND" => OpCode::AND,
        "OR" => OpCode::OR,
        "XOR" => OpCode::XOR,
        _ => panic!("Unknown opcode!"),
      };
      let b = subsplit.last().unwrap();

      let output = s.last().unwrap();

      gates.push(Gate { a: a.to_string(), b: b.to_string(), operation: opcode, output: output.to_string(), processed: false, value: 0 });
    }
  }

  Device { registers, gates }
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
