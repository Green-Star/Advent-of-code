use std::result;

use crate::core::parse_number_list;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let machine = transform_data(data);

  println!("{:?}", machine);

  let mut finished_machine;
  let mut i = 0;
  let mut i = (0 * 8) + 3 * 8;
  let mut i = ((0 * 8) + 3 * 8) * 8 + 4 * 8;
  let mut i = (((0 * 8) + 3 * 8) * 8 + 4 * 8) * 8 + 5 * 8;
  let mut i = ((((0 * 8) + 3 * 8) * 8 + 4 * 8) * 8 + 5 * 8) * 8 + 3 * 8;
  let mut i = (((((0 * 8) + 3 * 8) * 8 + 4 * 8) * 8 + 5 * 8) * 8 + 3 * 8) * 8 + 0 * 8;
//  let mut i = ((1 * 8 + 3 * 8) * 8 + 4 * 8) * 8 + 5 * 8;
//  let mut i = ((24) * 8 + 4 * 8) * 8 + 5 * 8;
//i = 1;
//i = i * 8 + 3 * 8;
  loop {
    break;

    if i % 100 == 0 { println!("{i}") }
    let mut test = machine.clone();
    test.a = i;
    match test.process_until_halt() {
      Some(machine) => { finished_machine = machine; println!("{i} - {}", finished_machine.get_output_string()); i += 1},
      None => i += 64,
    }

    /* Toutes les puissance de 8, on gagne un output de plus */
    /* Comment on détermine sa valeur ? */
    /* Exemple:  */
    /* 224 - 4,3,0
      1792 (224*8) - 0,4,3,0
      14336 (224 * 8 * 8) - 0,0,4,3,0
    */
    /* Au sein d'une puissance de 8, on peut faire des + 8 pour ajuster la premiere valeur */

    /**** */
    /* En somme, tu prends l'output a l'envers */
    /* i = 0 */
    /* Ensuite, Pour chaques output -> i = (i * 8) + output * 8 */
    /* A la fin, tu obtiens dans i, le nombre que tu cherches ? */
    /* A voir demain, mais si c'est ca c'est facile */
    /**** */

    if i > 225 { break; }
    break;
  }

  let mut i = 0;
  for o in machine.opcodes.iter().rev() {
    i = i * 8 + o * 8;
  }
  let final_result = i;

  let mut test = machine.clone();
  test.a = i;
  let test = test.process_until_halt().unwrap();
  println!("{:?}", test);


//  let finished_machine = machine.process_until_halt();
//  println!("{} - {:?}", i, finished_machine);

  let final_result = i;
  println!("Part 2 final result: {}", final_result);
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
  fn process_until_halt(&self) -> Option<StateMachine> {
    let mut result = self.clone();

    loop {
      match result.opcodes.get(result.instruction) {
        Some(op) => {
          let instruction = *op;
          let operand = result.opcodes[result.instruction + 1];
          result.instruction += 2;
          result = result.process_next_instruction(instruction, operand);
/*
          if result.outputs.len() > result.opcodes.len() { return None }
          for i in 0..result.outputs.len() {
            if result.outputs[i] != result.opcodes[i] { return None }
          }
          */
        },
        None => break
      }
    }

    Some(result)
/*
    if result.opcodes == result.outputs { Some(result) }
    else { None }
    */
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
