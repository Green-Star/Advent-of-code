use std::collections::VecDeque;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let mut maze = transform_data(data);

  maze.explore();

  if let Some(final_result) = maze.map[maze.ending_position.0][maze.ending_position.1].exploring_score {
    println!("Part 1 final result: {}", final_result);
  } else {
    println!("No result!");
  }
}

fn transform_data(data: Vec<String>) -> Maze {
  let mut map = vec![];
  let mut ending_position = (0, 0);
  let mut explorer = VecDeque::new();
  let mut x = 0;

  for s in data {
    let mut line = vec![];
    let mut y = 0;
    for c in s.chars() {
      match c {
        '#' => { line.push(Tile { content: Some(Content::Wall), exploring_score: None }); },
        'E' => { line.push(Tile { content: None, exploring_score: None }); ending_position = (x, y); },
        'S' => { line.push(Tile { content: None, exploring_score: None }); explorer.push_back(Explorer { position: (x, y), direction: Direction::East, exploring_score: 0 }); },
        _ => { line.push(Tile { content: None, exploring_score: None }); }
      }
      y += 1;
    }
    map.push(line);
    x += 1;
  }

  Maze { map, ending_position, yet_to_explore: explorer }
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
  fn turn_left(&self) -> (Direction, (isize, isize)) {
    let new_direction = match self {
        Self::North => Self::East,
        Self::East => Self::South,
        Self::South => Self::West,
        Self::West => Self::North,
      };
    (new_direction, new_direction.offset())
  }
  fn turn_right(&self) -> (Direction, (isize, isize)) {
    let new_direction = match self {
      Self::North => Self::West,
      Self::East => Self::North,
      Self::South => Self::East,
      Self::West => Self::South,
    };
    (new_direction, new_direction.offset())
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Content {
  Wall,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Tile {
  content: Option<Content>,
  exploring_score: Option<i32>,
}

#[derive(Debug, Clone)]
struct Maze {
  map: Vec<Vec<Tile>>,
  ending_position: (usize, usize),

  yet_to_explore: VecDeque<Explorer>,
}
impl Maze {
  fn explore(&mut self) {
    loop {
      self.yet_to_explore.make_contiguous().sort_by(|a, b| a.exploring_score.cmp(&b.exploring_score));
      match self.yet_to_explore.pop_front() {
        Some(e) => {  print_exploring(&self, &e); self.explore_path(e) },
        None => break,
      }
    }
  }

  fn explore_path(&mut self, e: Explorer) {
    /* Stop as soon as we bump into a wall */
    match self.map[e.position.0][e.position.1].content {
      Some(Content::Wall) => return,
      None => {},
    }

    match self.map[e.position.0][e.position.1].exploring_score {
      Some(score) => if e.exploring_score >= score { return },
      None => {},
    }
    self.map[e.position.0][e.position.1].exploring_score = Some(e.exploring_score);

    match self.map[self.ending_position.0][self.ending_position.1].exploring_score {
      Some(score) => if e.exploring_score >= score { return },
      None => {},
    }

    let left = e.turn_left();
    match self.map[left.position.0][left.position.1].content {
      None => self.yet_to_explore.push_back(left),
      _ => {},
    }
    let right = e.turn_right();
    match self.map[right.position.0][right.position.1].content {
      None => self.yet_to_explore.push_back(right),
      _ => {},
    }

    let next_position = (e.position.0.checked_add_signed(e.direction.offset().0).unwrap(), e.position.1.checked_add_signed(e.direction.offset().1).unwrap());
    self.explore_path(Explorer { direction: e.direction, position: next_position, exploring_score: e.exploring_score + 1 });
  }
}

fn print_exploring(maze: &Maze, explorer: &Explorer) {
  println!("Current exploration: {} - {:?}", explorer.exploring_score, maze.map[maze.ending_position.0][maze.ending_position.1].exploring_score);
  for i in 0..maze.map.len() {
    for j in 0..maze.map[i].len() {
      if (explorer.position.0, explorer.position.1) == (i, j) {
        match explorer.direction {
          Direction::North => print!("^"),
          Direction::East => print!(">"),
          Direction::South => print!("v"),
          Direction::West => print!("<"),
        }
      } else if (maze.ending_position.0, maze.ending_position.1) == (i, j) {
          print!("E");
      } else {
        match maze.map[i][j].content {
          Some(Content::Wall) => print!("#"),
          None => { if let Some(_) = maze.map[i][j].exploring_score { print!("X"); } else { print!("."); }},
        }
      }
    }
    println!("");
  }
  println!("{:?}", maze.yet_to_explore);
  println!("");
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Explorer {
  position: (usize, usize),
  direction: Direction,
  exploring_score: i32,
}
impl Explorer {
  fn turn_left(&self) -> Explorer {
    let (new_direction, offset) = self.direction.turn_left();
    let new_position = (self.position.0.checked_add_signed(offset.0).unwrap(), self.position.1.checked_add_signed(offset.1).unwrap()); // Being optimistic here: no check for limitations
    let new_exploring_score = self.exploring_score + 1000 + 1;

    Explorer { direction: new_direction, position: new_position, exploring_score: new_exploring_score }
  }
  fn turn_right(&self) -> Explorer {
    let (new_direction, offset) = self.direction.turn_right();
    let new_position = (self.position.0.checked_add_signed(offset.0).unwrap(), self.position.1.checked_add_signed(offset.1).unwrap()); // Being optimistic here: no check for limitations
    let new_exploring_score = self.exploring_score + 1000 + 1;

    Explorer { direction: new_direction, position: new_position, exploring_score: new_exploring_score }
  }
}
