use std::collections::VecDeque;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let mut maze = transform_data(data);

  maze.explore();
  print_explored_maze(&maze);

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

fn print_explored_maze(maze: &Maze) {
  for i in 0..maze.map.len() {
    for j in 0..maze.map[i].len() {
      match maze.map[i][j].content {
        Some(Content::Wall) => print!("[ # ]"),
        None => {
          match maze.map[i][j].exploring_score {
            Some(score) => print!("[{:03}]", score),
            None => print!("[   ]"),
          }
        }
      }
    }
    println!("");
  }
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
    /* I will do sort of a Dijkstra algorithm here */
    loop {
      self.yet_to_explore.make_contiguous().sort_by(|a, b| a.exploring_score.cmp(&b.exploring_score));
      match self.yet_to_explore.pop_front() {
        /* Grab the closest from start yet-to-explore path (filled with the starting tile at beginning) and explore it straight ahead */
        Some(e) => self.explore_path(e),
        /* If there isn't any path to explore, we're finished */
        None => break,
      }
    }
  }

  /* Explore one path, straight ahead, recording all connected paths to it */
  fn explore_path(&mut self, e: Explorer) {
    /* Explore the current tile: */
    /* If the tile have already been explored by another path which was closer of the starting tile: stop here (we're not on the shortest path - no need to go further) */
    match self.map[e.position.0][e.position.1].exploring_score {
      Some(score) => if e.exploring_score > score { return },
      None => {},
    }
    /* Record the exploring score on the tile */
    self.map[e.position.0][e.position.1].exploring_score = Some(e.exploring_score);

    /* If we already found a path to the end of the maze, and this tile is already beyond this distance, stop here (we're already too far) */
    /* Note that this way, we'll stop as soon as we reach the ending tile */
    match self.map[self.ending_position.0][self.ending_position.1].exploring_score {
      Some(score) => if e.exploring_score >= score { return },
      None => {},
    }

    /* Let's check the other vertices on this tile */
    for d in vec![Direction::North, Direction::East, Direction::West, Direction::South] {
      let (next_x, next_y) = (e.position.0.checked_add_signed(d.offset().0).unwrap(), e.position.1.checked_add_signed(d.offset().1).unwrap());
      match self.map[next_x][next_y].content {
        None => self.yet_to_explore.push_back(Explorer { direction: e.direction, position: (next_x, next_y), exploring_score: e.exploring_score + 1 }),
        _ => {}
      }
    }
/*
    let (north_x, north_y) =

    /* For both left and right, if there is another path starting from this tile (i.e. the tile on the left - or right - is not a wall), record it the yet-to-explore vector */
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

    /* And now, let's focus on our path... */
    let next_position = (e.position.0.checked_add_signed(e.direction.offset().0).unwrap(), e.position.1.checked_add_signed(e.direction.offset().1).unwrap());
    /* Check next tile: */
    let move_on = {
      match self.map[next_position.0][next_position.1].content {
        Some(Content::Wall) => false,
        None => true,
      }
    };
    if move_on {
      /* If we can go onto this straight line, let's move on by going one tile forward */
      self.explore_path(Explorer { direction: e.direction, position: next_position, exploring_score: e.exploring_score + 1 });
    } else {
      /* If we're heading to the wall, then stop here, and add 1000 to the exploring score of the tile (because we'll have to turn on this tile) */
      self.map[e.position.0][e.position.1].exploring_score = Some(e.exploring_score + 1000);
    }
    */
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
