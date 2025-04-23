use std::{collections::{HashMap, HashSet}, os::unix::raw::gid_t};

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let device = transform_data(data);

  let final_result = device.get_final_result();
  println!("Part 2 final result: {}", final_result);
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


#[derive(Debug, Clone, PartialEq)]
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
  /*
    1. If the output of a gate is z, then the operation has to be XOR unless it is the last bit.
    2. If the output of a gate is not z and the inputs are not x, y then it has to be AND / OR gate, but not XOR gate.
    3. If you have a XOR gate with inputs x, y, there must be another XOR gate with the output of this gate as an input. Search through all gates for an XOR-gate with this gate as an input; if it does not exist, your (original) XOR gate is faulty.
    4. Similarly, if you have an AND gate, there must be an OR gate with this gate as an input. If that gate doesn’t exist, the original AND gate is faulty.
    (These don't apply for the gates with input x00, y00).

  */
  fn find_swapped(&self) -> Vec<String> {
    let mut final_bytes: Vec<_> = self.registers.keys().filter(|register| register.starts_with("z")).collect();
    final_bytes.sort_by(|x, y| y.cmp(x));
    let final_bit = final_bytes[0];
    println!("Final bit: {final_bit}");


    /* 1. If the output of a gate is z, then the operation has to be XOR unless it is the last bit. */
    let a = self.gates.iter()
              .filter(|gate| gate.output.starts_with("z"))
              .filter(|gate| gate.output != *final_bit)
              .filter(|gate| gate.operation != OpCode::XOR)
              .collect::<Vec<_>>();
    println!("A: {:?}", a);

    /* 2. If the output of a gate is not z and the inputs are not x, y then it has to be AND / OR gate, but not XOR gate. */
    let b = self.gates.iter()
              .filter(|gate| gate.output.starts_with("z") == false)
              .filter(|gate| gate.a.starts_with("x") == false && gate.b.starts_with("x") == false)
              .filter(|gate| gate.a.starts_with("y") == false && gate.b.starts_with("y") == false)
              .filter(|gate| gate.operation == OpCode::XOR)
              .collect::<Vec<_>>();
    println!("B: {:?}", b);

    /* 3. If you have a XOR gate with inputs x, y, there must be another XOR gate with the output of this gate as an input. Search through all gates for an XOR-gate with this gate as an input; if it does not exist, your (original) XOR gate is faulty. */
    let c= self.gates.iter()
              .filter(|gate| gate.operation == OpCode::XOR)
              .filter(|gate| gate.a != "x00" && gate.a != "y00")
              .filter(|gate| gate.b != "x00" && gate.b != "y00")
              .filter(|gate| gate.a.starts_with("x") || gate.a.starts_with("y"))
              .filter(|gate| gate.b.starts_with("x") || gate.b.starts_with("y"))
              .filter(|gate| (self.gates.iter().filter(|g| g.operation == OpCode::XOR).filter(|g| g.a == gate.output || g.b == gate.output).collect::<Vec<_>>().is_empty()) )
              .collect::<Vec<_>>();
    println!("C: {:?}", c);

    /* 4. Similarly, if you have an AND gate, there must be an OR gate with this gate as an input. If that gate doesn’t exist, the original AND gate is faulty. */
    let d = self.gates.iter()
              .filter(|gate| gate.operation == OpCode::AND)
              .filter(|gate| gate.a != "x00" && gate.a != "y00")
              .filter(|gate| gate.b != "x00" && gate.b != "y00")
              .filter(|gate| (self.gates.iter().filter(|g| g.operation == OpCode::OR).any(|g| g.a == gate.output || g.b == gate.output)) == false)
              .collect::<Vec<_>>();
    println!("D: {:?}", d);

    let result_set = a.iter().chain(b.iter()).chain(c.iter()).chain(d.iter())
                                          .map(|gate| gate.output.clone())
                                          .collect::<HashSet<_>>();
    result_set.into_iter().collect::<Vec<_>>()
  }

  fn get_final_result(&self) -> String {
    /* find swapped ->  sort and join */
    let mut swapped_gates = self.find_swapped();
    swapped_gates.sort();
    swapped_gates.join(",")
  }
}
