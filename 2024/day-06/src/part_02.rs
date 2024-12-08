pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let data = transform_data(data);

  let mut lab = create_lab(&data);
  lab.patrol();

  let mut infinite_labs = Vec::new();
  for i in 0..lab.map.len() {
    for j in 0..lab.map[i].len() {
      match lab.map[i][j].patrolled {
          Some(_) => infinite_labs.push(create_infinite_lab(&data, &(i, j))),
          _ => {},
      }
    }
  }

  let mut final_result = 0;
  for mut l in infinite_labs {
    l.infinite_patrol();
    if l.infinite_loop { final_result += 1 }
  }

  println!("Part 2 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> Vec<Vec<char>> {
  let mut result = Vec::new();

  for line in data {
    let mut chars = Vec::new();
    for c in line.chars() {
      chars.push(c);
    }
    result.push(chars);
  }

  result
}

fn create_lab(data: &Vec<Vec<char>>) -> Lab {
  let mut map = Vec::new();
  let mut guard = Guard { direction: Direction::North, position: (0, 0)};

  for i in 0..data.len() {
    let mut line = Vec::new();
    for j in 0..data[i].len() {
      let mut pos = Position { c: data[i][j], patrolled: None };
      if data[i][j] == '^' {
        guard = Guard { direction: Direction::North, position: (i, j) };
        pos = Position { c: data[i][j], patrolled: Some('X') };
      }
      line.push(pos);
    }
    map.push(line);
  }

  Lab { map: map, guard: guard }
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
          Direction::North => (-1, 0),
          Direction::East => (0, 1),
          Direction::South => (1, 0),
          Direction::West => (0, -1),
      }
    }
}
#[derive(Debug, Copy, Clone)]
struct Guard {
  direction: Direction,
  position: (usize, usize),
}
impl Guard {
    fn turn_right(&mut self) {
      match self.direction {
          Direction::North => self.direction = Direction::East,
          Direction::East => self.direction = Direction::South,
          Direction::South => self.direction = Direction::West,
          Direction::West => self.direction = Direction::North,
      }
    }
}

#[derive(Debug, Clone)]
struct InfinitePosition {
  c: char,
  patrolled: Vec<Direction>,
}

#[derive(Debug, Clone)]
struct InfiniteLab {
  map: Vec<Vec<InfinitePosition>>,
  guard: Guard,
  infinite_loop: bool,
}
impl InfiniteLab {
    fn infinite_patrol(&mut self) {
      let offset = self.guard.direction.offset();

      let next_guard_index = (self.guard.position.0.checked_add_signed(offset.0), self.guard.position.1.checked_add_signed(offset.1));
      let x;
      let y;
      match next_guard_index {
          (Some(i), Some(j)) => {
            if i >= self.map.len() { return }
            if j >= self.map[i].len() { return }
            x = i;
            y = j;
          },
          _ => return,
      }

      match self.map[x][y].c {
        '#' => self.guard.turn_right(),
        _ => {
          if self.map[x][y].patrolled.contains(&(self.guard.direction)) { self.infinite_loop = true; return }

          self.guard.position = (x, y);
          self.map[self.guard.position.0][self.guard.position.1].patrolled.push(self.guard.direction);
        },
      }

      self.infinite_patrol()
    }
}
fn create_infinite_lab(data: &Vec<Vec<char>>, new_obstacle_index: &(usize, usize)) -> InfiniteLab {
  let mut map = Vec::new();
  let mut guard = Guard { direction: Direction::North, position: (0, 0)};

  for i in 0..data.len() {
    let mut line = Vec::new();
    for j in 0..data[i].len() {
      let mut pos = InfinitePosition { c: data[i][j], patrolled: vec![] };
      if data[i][j] == '^' {
        guard = Guard { direction: Direction::North, position: (i, j) };
        pos = InfinitePosition { c: data[i][j], patrolled: vec![guard.direction] };
      }
      line.push(pos);
    }
    map.push(line);
  }
  map[new_obstacle_index.0][new_obstacle_index.1].c = '#';

  InfiniteLab { map: map, guard: guard, infinite_loop: false }
}

#[derive(Debug, Copy, Clone)]
struct Position {
  c: char,
  patrolled: Option<char>,
}

#[derive(Debug, Clone)]
struct Lab {
  map: Vec<Vec<Position>>,
  guard: Guard,
}
impl Lab {
    fn patrol(&mut self) {
      let offset = self.guard.direction.offset();

      let next_guard_index = (self.guard.position.0.checked_add_signed(offset.0), self.guard.position.1.checked_add_signed(offset.1));
      let x;
      let y;
      match next_guard_index {
          (Some(i), Some(j)) => {
            if i >= self.map.len() { return }
            if j >= self.map[i].len() { return }
            x = i;
            y = j;
          },
          _ => return,
      }

      match self.map[x][y].c {
        '#' => self.guard.turn_right(),
        _ => {
          self.guard.position = (x, y);
          self.map[self.guard.position.0][self.guard.position.1].patrolled = Some('X');
        },
      }

      self.patrol()
    }
}
