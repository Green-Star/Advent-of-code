pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let (mut warehouse, directions) = transform_data(data);

  warehouse.print();
  println!("*****");
  /*
  for d in directions {
    warehouse.do_move(d);
    }
    warehouse.print();
    */
    warehouse.do_move(Direction::West);
    warehouse.print();
    println!("*****");
    warehouse.do_move(Direction::South);
    warehouse.print();
    println!("*****");
    warehouse.do_move(Direction::South);
    warehouse.print();
    println!("*****");
    warehouse.do_move(Direction::West);
    warehouse.print();
    println!("*****");
    warehouse.do_move(Direction::West);
    warehouse.print();
    println!("*****");
    warehouse.do_move(Direction::North);
    warehouse.print();
    println!("*****");
    warehouse.do_move(Direction::North);
    warehouse.print();
    println!("*****");
    warehouse.do_move(Direction::West);
    warehouse.print();
    println!("*****");
    warehouse.do_move(Direction::West);
    warehouse.print();
    println!("*****");
    warehouse.do_move(Direction::North);
    warehouse.print();
    println!("*****");
    warehouse.do_move(Direction::North);
    warehouse.print();
    println!("*****");


  let final_result = warehouse.gps();

  println!("Part 2 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> (Warehouse, Vec<Direction>) {
  let mut map = vec![];
  let mut map_description = true;
  let mut directions = vec![];
  let mut robot_position = (0, 0);
  let mut i = 0;

  for line in data {
    if line.is_empty() { map_description = false; }

    if map_description {
      let mut l = vec![];
      let mut j = 0;
      for c in line.chars() {
        match c {
          '#' => { l.push(Some(Content::Wall)); l.push(Some(Content::Wall)); },
          'O' => { l.push(Some(Content::LeftBoxEdge)); l.push(Some(Content::RightBoxEdge)); },
          '@' => { l.push(Some(Content::Robot)); l.push(None); robot_position = (i, j) },
          _ => { l.push(None); l.push(None); },
        }
        j += 2;
      }
      map.push(l);
      i += 1;
    } else {
      for c in line.chars() {
        match c {
          '^' => { directions.push(Direction::North) },
          'v' => { directions.push(Direction::South) },
          '<' => { directions.push(Direction::West) },
          '>' => { directions.push(Direction::East) },
          _ => {},
        }
      }
    }
  }

  (Warehouse { map, robot_position, last_failed_movement: None }, directions)
}


#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
  North,
  East,
  South,
  West,
}
impl Direction {
  fn offset(&self) -> (isize, isize) {
    match self {
      Self::North => { (-1, 0) },
      Self::East => { (0, 1) },
      Self::South => { (1, 0) },
      Self::West => { (0, -1) },
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Content {
  Wall,
  LeftBoxEdge,
  RightBoxEdge,
  Robot,
}

#[derive(Debug, Clone)]
struct Warehouse {
  map: Vec<Vec<Option<Content>>>,
  robot_position: (usize, usize),

  last_failed_movement: Option<Direction>,
}
impl Warehouse {
  fn do_move(&mut self, direction: Direction) {
    match self.last_failed_movement {
      Some(d) => if d == direction { return },
      None => {},
    }

    let offset = direction.offset();
    let moved;
    match direction {
      Direction::North | Direction::South => { moved = self.shift_up_or_down(self.robot_position, offset); },
      Direction::West | Direction::East => { moved = self.shift(self.robot_position, offset); },
    }

    if moved {
      self.robot_position = (self.robot_position.0.checked_add_signed(offset.0).unwrap(), self.robot_position.1.checked_add_signed(offset.1).unwrap());
      self.last_failed_movement = None;
    } else {
      self.last_failed_movement = Some(direction);
    }
  }

  fn shift(&mut self, index: (usize, usize), direction: (isize, isize)) -> bool {
    /* Just a useless quick check */
    if index.0 >= self.map.len() { return false; }
    if index.1 >= self.map[index.0].len() { return false; }

    match self.map[index.0][index.1] {
      None => { return true; },
      Some(s) => {
        if s == Content::Wall { return false; }

        let neighbour = (index.0.checked_add_signed(direction.0).unwrap(), index.1.checked_add_signed(direction.1).unwrap());
        let can_move = self.shift(neighbour, direction);

        if can_move {
          self.map[neighbour.0][neighbour.1] = self.map[index.0][index.1];
          self.map[index.0][index.1] = None;
        }

        return can_move;
      }
    }
  }

  fn shift_up_or_down(&mut self, index: (usize, usize), direction: (isize, isize)) -> bool {
    let next_index = (index.0.checked_add_signed(direction.0).unwrap(), index.1.checked_add_signed(direction.1).unwrap());
    let can_move = self.can_move_up_or_down(next_index, direction);

    println!("Can move: {}", can_move);

    if can_move {
      self.do_shift_up_or_down(next_index, direction);
      self.map[next_index.0][next_index.1] = self.map[index.0][index.1];
      self.map[index.0][index.1] = None;
    }

    can_move
  }
  fn do_shift_up_or_down(&mut self, index: (usize, usize), direction: (isize, isize)) {
    let next_index = (index.0.checked_add_signed(direction.0).unwrap(), index.1.checked_add_signed(direction.1).unwrap());
    match self.map[index.0][index.1] {
      None | Some(Content::Wall) => {},
      Some(Content::LeftBoxEdge) => {
        self.do_shift_up_or_down(next_index, direction);
        self.map[next_index.0][next_index.1] = self.map[index.0][index.1];
        self.map[index.0][index.1] = None;
        self.do_shift_up_or_down((index.0, index.1.checked_add_signed(1).unwrap()), direction);
      },
      Some(Content::RightBoxEdge) => {
        self.do_shift_up_or_down(next_index, direction);
        self.map[next_index.0][next_index.1] = self.map[index.0][index.1];
        self.map[index.0][index.1] = None;
        self.do_shift_up_or_down((index.0, index.1.checked_add_signed(-1).unwrap()), direction);
      },
      Some(Content::Robot) => {
      }
    }
  }

  fn can_move_up_or_down(&mut self, index: (usize, usize), direction: (isize, isize)) -> bool {
    /* Just a useless quick check */
    if index.0 >= self.map.len() { return false; }
    if index.1 >= self.map[index.0].len() { return false; }

    match self.map[index.0][index.1] {
      None => { return true; },
      Some(Content::Wall) => { return false; },
      Some(s) => {
        let neighbour = (index.0.checked_add_signed(direction.0).unwrap(), index.1.checked_add_signed(direction.1).unwrap());
        let other_edge;
        if s == Content::LeftBoxEdge {
          other_edge = (index.0.checked_add_signed(direction.0).unwrap(), index.1.checked_add_signed(direction.1 + 1).unwrap());
        } else if s == Content::RightBoxEdge {
          other_edge = (index.0.checked_add_signed(direction.0).unwrap(), index.1.checked_add_signed(direction.1 - 1).unwrap());
        } else {
          // Should not happen
          other_edge = neighbour;
        }

        let can_move = self.can_move_up_or_down(neighbour, direction);
        let other_edge_can_move = self.can_move_up_or_down(other_edge, direction);

        return can_move && other_edge_can_move;
      }
    }
  }

  fn gps(&self) -> i32 {
    let mut score = 0;

    for i in 1..self.map.len() {
      for j in 1..self.map[i].len() {
        match self.map[i][j] {
          Some(Content::LeftBoxEdge) => { score += (i * 100 + j) as i32 },
          _ => {},
        }
      }
    }

    score
  }

  fn print(&self) {
    for i in 0..self.map.len() {
      for j in 0..self.map[i].len() {
        match self.map[i][j] {
          Some(Content::Wall) => { print!("#") },
          Some(Content::LeftBoxEdge) => { print!("[") },
          Some(Content::RightBoxEdge) => { print!("]") },
          Some(Content::Robot) => { print!("@") },
          None => { print!(".") },
        }
      }
      println!("");
    }
  }
}
