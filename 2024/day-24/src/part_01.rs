use std::{collections::HashMap, result};

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let mut device = transform_data(data);
  println!("{:?}", device);

  device.process_until_halt();
  println!("{:?}", device);

  let final_result = device.get_final_result();
  println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> Device {
  let mut registers = HashMap::new();
  let mut gates = vec![];

  for line in data {
    if line.contains(": ") {
      let mut s = line.split(": ");

      let register = s.next().unwrap();
      let value: i64 = s.last().unwrap().parse().unwrap();

      registers.insert(register.to_string(), Some(value));
    }
    else if line.contains(" -> ") {
      let mut s = line.split(" -> ");
      let gate = s.next().unwrap();

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

      registers.entry(a.to_string()).or_insert(None);
      registers.entry(b.to_string()).or_insert(None);
      registers.entry(output.to_string()).or_insert(None);
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
  fn process(&self, x: &i64, y: &i64) -> i64 {
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
impl Device {
  fn process_until_halt(&mut self) {
    while self.registers.iter().filter(|(key, _)| key.starts_with("z")).any(|(_, value)| value.is_none()) {
      self.gates.iter_mut()
                .filter(|gate| gate.processed == false)
                .for_each(|gate| {
                  if let Some(a) = self.registers.get(&gate.a).unwrap() {
                    if let Some(b) = self.registers.get(&gate.b).unwrap() {
                      println!("Processing {:?}", gate);
                      let out = gate.operation.process(a, b);

                      self.registers.entry(gate.output.clone()).and_modify(|e| *e = Some(out));

                      gate.value = out;
                      gate.processed = true;
                    }
                  }
                });
    }
  }
  fn get_final_result(&self) -> i64 {
    self.registers.iter()
                  .filter(|(k, _)| k.starts_with("z"))
                  .map(|(k, v)| {
                    let i = String::from_iter(k.chars().skip(1)).parse().unwrap();
                    let byte_value = v.unwrap() * (2 as i64).pow(i);
                    byte_value
                  })
                  .sum()
  }
}
