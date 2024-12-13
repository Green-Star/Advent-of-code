pub fn resolve(input_data_path: &str) {
    // let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    // let (map, empty_map) = transform_data(data);

    // let final_result = do_stuff(map, &empty_map);

    // println!("Part 1 final result: {}", final_result);

  println!("{:?}", resolve_system(Position {x: 94, y: 34}, Position {x: 22, y: 67}, Position {x:8400, y:5400}));

  println!("{:?}", resolve_system(Position {x: 26, y: 66}, Position {x: 67, y: 21}, Position {x:12748, y:12176}));

  println!("{:?}", resolve_system(Position {x: 17, y: 86}, Position {x: 84, y: 37}, Position {x:7870, y:6450}));

  println!("{:?}", resolve_system(Position {x: 69, y: 23}, Position {x: 27, y: 71}, Position {x:18641, y:10279}));

}

#[derive(Debug, Copy, Clone)]
struct Position {
  x: i32,
  y: i32,
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
pub fn resolve_system(a_offset: Position, b_offest: Position, prize_position: Position) -> Option<(i32, i32)> {
    let A = ((prize_position.x * b_offest.y) - (b_offest.x * prize_position.y)) / ((a_offset.x * b_offest.y) - (b_offest.x * a_offset.y));
    let B = ((a_offset.x * prize_position.y) - (prize_position.x * a_offset.y)) / ((a_offset.x * b_offest.y) - (b_offest.x * a_offset.y));

    let x_position = A * a_offset.x + B * b_offest.x;
    let y_position = A * a_offset.y + B * b_offest.y;

    if (x_position, y_position) == (prize_position.x, prize_position.y) {
      Some((A, B))
    } else {
      None
    }
}
