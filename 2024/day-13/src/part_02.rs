pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let systems = transform_data(data);

    let final_result: i128 = systems.iter().map(|s| resolve_system(s.0, s.1, s.2)).map(|o| get_token_cost(o)).sum();

    println!("Part 2 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> Vec<(Position, Position, Position)> {
  let mut result = vec![];

  let mut a_offset = Position { x: 0, y: 0};
  let mut b_offset = Position { x: 0, y: 0};

  for line in data {
    if line.is_empty() { continue }

    if line.starts_with("Button A:") {
      let (x_offset, y_offset) = parse_button_description(&line);
      a_offset = Position { x: x_offset, y: y_offset };
    } else if line.starts_with("Button B:") {
      let (x_offset, y_offset) = parse_button_description(&line);
      b_offset = Position { x: x_offset, y: y_offset };
    } else if line.starts_with("Prize:") {
      let s = line.split(": ");
      let mut sub = s.last().unwrap().split(", ");
      let (x, y) = (sub.next().unwrap(), sub.last().unwrap());
      let (x, y): (i128, i128) = (x.split("=").last().unwrap().parse().unwrap(), y.split("=").last().unwrap().parse().unwrap());
      let prize_position = Position { x: 10000000000000 + x, y: 10000000000000 + y };

      result.push((a_offset, b_offset, prize_position));
    }
  }

  result
}

fn parse_button_description(line: &str) -> (i128, i128) {
  let s = line.split(": ");
  let mut sub = s.last().unwrap().split(", ");
  let (x, y) = (sub.next().unwrap(), sub.last().unwrap());
  (parse_offset(x), parse_offset(y))
}

fn parse_offset(offset: &str) -> i128 {
  let s = offset.split("+");

  s.last().unwrap().parse().unwrap()
}

#[derive(Debug, Copy, Clone)]
struct Position {
  x: i128,
  y: i128,
}

/*
Thanks to `https://en.wikipedia.org/wiki/Cramer's_rule#Applications``

Given the input:
  Button A: X+94, Y+34
  Button B: X+22, Y+67
  Prize: X=8400, Y=5400

We have the linear system:
  (X+94)*x + (X*22)*y = 8400
  (Y+34)*x + (Y+67)*y = 5400

Which is the matrix:
  (X+94) (X*22)    x   8400
  (Y+34) (Y+67)    y = 5400

So, the solution is:
  x = (8400*(Y+67) - (X*22)*5400) / (X+94 * Y+67) - (X+22 * Y+34)
  y = ((X+94) * 5400 - 8400 * (Y+34)) / ((X+94 * Y+67) - (X+22 * Y+34))
*/
fn resolve_system(a_offset: Position, b_offest: Position, prize_position: Position) -> Option<(i128, i128)> {
    let a = ((prize_position.x * b_offest.y) - (b_offest.x * prize_position.y)) / ((a_offset.x * b_offest.y) - (b_offest.x * a_offset.y));
    let b = ((a_offset.x * prize_position.y) - (prize_position.x * a_offset.y)) / ((a_offset.x * b_offest.y) - (b_offest.x * a_offset.y));

    let x_position = a * a_offset.x + b * b_offest.x;
    let y_position = a * a_offset.y + b * b_offest.y;

    if (x_position, y_position) == (prize_position.x, prize_position.y) {
      Some((a, b))
    } else {
      None
    }
}

fn get_token_cost(times: Option<(i128, i128)>) -> i128 {
  match times {
    Some((a, b)) => a * 3 + b,
    None => 0
  }
}