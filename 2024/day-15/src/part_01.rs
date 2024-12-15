pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let (mut warehouse, directions) = transform_data(data);

  warehouse.print();
  println!("*****");

  for d in directions {
    warehouse.do_move(d);
  }
  warehouse.print();

  let final_result = warehouse.gps();

  println!("Part 1 final result: {}", final_result);
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
          '#' => { l.push(Some(Content::Wall)) },
          'O' => { l.push(Some(Content::Box)) },
          '@' => { l.push(Some(Content::Robot)); robot_position = (i, j) },
          _ => { l.push(None) },
        }
        j += 1;
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
  Box,
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

    let moved = self.shift(self.robot_position, offset);
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

  fn gps(&self) -> i32 {
    let mut score = 0;

    for i in 1..self.map.len() {
      for j in 1..self.map[i].len() {
        match self.map[i][j] {
          Some(Content::Box) => { score += (i * 100 + j) as i32 },
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
          Some(Content::Wall) => { print!("[#]") },
          Some(Content::Box) => { print!("[O]") },
          Some(Content::Robot) => { print!("[@]") },
          None => { print!("[.]") },
        }
      }
      println!("");
    }
  }
}
