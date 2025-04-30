use std::collections::{HashSet, VecDeque};

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let mut maze = transform_data(data);

  maze.explore();
  maze.find_best_path();
  print_explored_maze(&maze);
//  println!("*****");
  print_best_path(&maze);
  let final_result: usize = maze.map.iter().map(|v| v.iter().filter(|t| t.is_best).collect::<Vec<&Tile>>()).map(|fv| fv.len()).sum();

  println!("Part 2 final result: {} (> 521)", final_result);
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
        '#' => { line.push(Tile { content: Some(Content::Wall), exploring_score: None, is_best: false, neighbours: HashSet::new() }); },
        'E' => { line.push(Tile { content: None, exploring_score: None, is_best: true, neighbours: HashSet::new() }); ending_position = (x, y); }, // Note: The ending tile is on the best path by default
        'S' => { line.push(Tile { content: None, exploring_score: Some((0, Direction::East)), is_best: false, neighbours: HashSet::new() }); explorer.push_back(Explorer { position: (x, y), direction: Direction::East, exploring_score: 0 }); },
        _ => { line.push(Tile { content: None, exploring_score: None, is_best: false, neighbours: HashSet::new() }); }
      }
      y += 1;
    }
    map.push(line);
    x += 1;
  }

  Maze { map, ending_position, yet_to_explore: explorer, explorer_done: HashSet::new() }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq)]
struct Tile {
  content: Option<Content>,
  exploring_score: Option<(i32, Direction)>,
  is_best: bool,

  neighbours: HashSet<(usize, usize)>
}

#[derive(Debug, Clone)]
struct Maze {
  map: Vec<Vec<Tile>>,
  ending_position: (usize, usize),

  yet_to_explore: VecDeque<Explorer>,
  explorer_done: HashSet<Explorer>,
}
impl Maze {
  fn explore(&mut self) {
    /* I will do sort of a Dijkstra algorithm here */
    loop {
      self.yet_to_explore.make_contiguous().sort_by(|a, b| a.exploring_score.cmp(&b.exploring_score));
      match self.yet_to_explore.pop_front() {
        /* Grab the closest from start yet-to-explore path (filled with the starting tile at beginning) and explore it straight ahead */
        Some(e) => { println!("Explorer: {:?}", e); self.explore_path(e) },
        /* If there isn't any path to explore, we're finished */
        None => break,
      }
    }
  }


  /* Explore one path, straight ahead, recording all connected paths to it */
  fn explore_path(&mut self, e: Explorer) {
    /* Explorer already done, stop immediately */
    if self.explorer_done.insert(e) == false { return }

    let left = e.turn_left();
    // Being optimistic here: no check for limitations
    match self.map[left.position.0.checked_add_signed(left.direction.offset().0).unwrap()][left.position.1.checked_add_signed(left.direction.offset().1).unwrap()].content {
      /* There's another path to explore, let's queue it (but don't go on it yet) */
      None => self.yet_to_explore.push_back(left),
      _ => {},
    }
    let right = e.turn_right();
    // Being optimistic here: no check for limitations
    match self.map[right.position.0.checked_add_signed(right.direction.offset().0).unwrap()][right.position.1.checked_add_signed(right.direction.offset().1).unwrap()].content {
      /* There's another path to explore, let's queue it (but don't gor forward yet) */
      None => self.yet_to_explore.push_back(right),
      _ => {},
    }

    /* And now, let's focus on our path... */
    // Again: no check for limitations
    let next_position = (e.position.0.checked_add_signed(e.direction.offset().0).unwrap(), e.position.1.checked_add_signed(e.direction.offset().1).unwrap());
    /* Bumped into a wall, stop here */
    if let Some(_) = self.map[next_position.0][next_position.1].content { return }

    let next_direction = e.direction;
    let next_score = e.exploring_score + 1;
    let next = Explorer { position: next_position, direction: next_direction, exploring_score: next_score };

    /* We're on an accessible tile, let's check its score */
    match self.map[next.position.0][next.position.1].exploring_score {
    /*  3 possibilites here: */
      Some((score, direction)) => {
        /* A REPRENDRE, JE CROIS QUE J'AI TOUT FAIT A L'ENVERS!!!  */


        /* 1. We're above the shortest path to this tile */
        /*  => Stop here, there's nothing left to on this tile */
        if next.exploring_score > score { return }

        /* 2. We're below the shortest path to this tile */
        /* Let's leveled our score compare to the record shortest path */
        let leveled_score = next.get_leveled_score(direction);
        /* If we're above the shortest score, then there is another shortest path to this tile and we stop here */
        if leveled_score > score { return }
        /* If we equal the shortest score, then have to record ourself as one the closest neighbour of the tile */
        if leveled_score == score {
          self.map[next_position.0][next_position.1].neighbours.insert(e.position);
        }
        /* If we're below the shortest score, we found a shortest path leading to this tile */
          /* => Update the score of this tile and record ourself as the only closest neighbour */
        if leveled_score < score {
          self.map[next.position.0][next.position.1].exploring_score = Some((next.exploring_score, next.direction));
          self.map[next.position.0][next.position.1].neighbours = HashSet::new();
          self.map[next.position.0][next.position.1].neighbours.insert(e.position);
        }
      },

      /* 3. No score: the tile wasn't explored yet, */
      /*  => Simply record ourself as the closest neighbour of the tile and set its score */
      None => {
        self.map[next.position.0][next.position.1].exploring_score = Some((next.exploring_score, next.direction));
        self.map[next.position.0][next.position.1].neighbours = HashSet::new();
        self.map[next.position.0][next.position.1].neighbours.insert(e.position);
      },
    }

    /* Small optimization, if we're already farther than the end tile, stop here */
    if let Some((final_score, _)) = self.map[self.ending_position.0][self.ending_position.1].exploring_score {
      if next.exploring_score > final_score { return }
    }

    /* Then, let's keep exploring this path (until we reach a wall) */
    self.explore_path(next);

  /***

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
    ***/
  }

  fn find_best_path(&mut self) {
    /* Good thing, we did a Dijkstra algorithm to find the shortest path between starting and ending points */
    /* Thus, we already know the best path(s): simply start from the ending position and reverse the path (following decreasing exploring_score) until we reach the starting point (which has exploring score 0) */
    let index = (self.ending_position.0, self.ending_position.1);
    // TODO
  }

/***
  fn find_best_path(&mut self) {
    /* Good thing, we did a Dijkstra algorithm to find the shortest path between starting and ending points */
    /* Thus, we already know the best path(s): simply start from the ending position and reverse the path (following decreasing exploring_score) until we reach the starting point (which has exploring score 0) */
    let index = (self.ending_position.0, self.ending_position.1);
    self.follow_one_path(index);
  }
  fn follow_one_path(&mut self, index: (usize, usize)) {
    let own_score = self.map[index.0][index.1].exploring_score.unwrap();

    let next_index = (index.0.checked_add_signed(Direction::North.offset().0).unwrap(), index.1.checked_add_signed(Direction::North.offset().1).unwrap());
    match self.map[next_index.0][next_index.1].exploring_score {
      Some(next_score) => {
        if next_score < own_score {
          self.map[next_index.0][next_index.1].is_best = true;
          self.follow_one_path(next_index);
        }
      },
      None => {},
    }

    let next_index = (index.0.checked_add_signed(Direction::East.offset().0).unwrap(), index.1.checked_add_signed(Direction::East.offset().1).unwrap());
    match self.map[next_index.0][next_index.1].exploring_score {
      Some(next_score) => {
        if next_score < own_score {
          self.map[next_index.0][next_index.1].is_best = true;
          self.follow_one_path(next_index);
        }
      },
      None => {},
    }

    let next_index = (index.0.checked_add_signed(Direction::South.offset().0).unwrap(), index.1.checked_add_signed(Direction::South.offset().1).unwrap());
    match self.map[next_index.0][next_index.1].exploring_score {
      Some(next_score) => {
        if next_score < own_score {
          self.map[next_index.0][next_index.1].is_best = true;
          self.follow_one_path(next_index);
        }
      },
      None => {},
    }

    let next_index = (index.0.checked_add_signed(Direction::West.offset().0).unwrap(), index.1.checked_add_signed(Direction::West.offset().1).unwrap());
    match self.map[next_index.0][next_index.1].exploring_score {
      Some(next_score) => {
        if next_score < own_score {
          self.map[next_index.0][next_index.1].is_best = true;
          self.follow_one_path(next_index);
        }
      },
      None => {},
    }
  }
  ***/
}

fn print_best_path(maze: &Maze) {
  for i in 0..maze.map.len() {
    for j in 0..maze.map[i].len() {
      if maze.map[i][j].is_best {
        print!("O");
      } else if let Some(Content::Wall) = maze.map[i][j].content {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!("");
  }
}
fn print_explored_maze(maze: &Maze) {
  for i in 0..maze.map.len() {
    for j in 0..maze.map[i].len() {
      match maze.map[i][j].content {
        Some(Content::Wall) => print!("[  #  ]"),
        None => {
          match maze.map[i][j].exploring_score {
            Some((score, _)) => print!("[{:05}]", score),
            None => print!("[     ]"),
          }
        }
      }
    }
    println!("");
  }

  let pos = maze.ending_position;
  println!("{:?}", maze.map[pos.0][pos.1]);
  let pos = maze.map[pos.0][pos.1].neighbours.iter().collect::<Vec<_>>()[0];
  println!("{:?}", maze.map[pos.0][pos.1]);
  let pos = maze.map[pos.0][pos.1].neighbours.iter().collect::<Vec<_>>()[0];
  println!("{:?}", maze.map[pos.0][pos.1]);
}

#[derive(Debug, Copy, Clone, Eq, Hash)]
struct Explorer {
  position: (usize, usize),
  direction: Direction,
  exploring_score: i32,
}
impl PartialEq for Explorer {
  fn eq(&self, other: &Self) -> bool {
      self.position == other.position && self.direction == other.direction
  }
}
impl Explorer {
  fn turn_left(&self) -> Explorer {
    let (new_direction, _) = self.direction.turn_left();
    let new_position = self.position;
    let new_exploring_score = self.exploring_score + 1000;

    Explorer { direction: new_direction, position: new_position, exploring_score: new_exploring_score }
  }
  fn turn_right(&self) -> Explorer {
    let (new_direction, _) = self.direction.turn_right();
    let new_position = self.position;
    let new_exploring_score = self.exploring_score + 1000;

    Explorer { direction: new_direction, position: new_position, exploring_score: new_exploring_score }
  }
  fn get_leveled_score(&self, other_direction: Direction) -> i32 {
    match self.direction {
      Direction::North => {
        match other_direction {
          Direction::North => { self.exploring_score },
          Direction::East | Direction::West => { self.exploring_score + 1000 },
          Direction::South => { self.exploring_score + 2000 },
        }
      },
      Direction::East => {
        match other_direction {
          Direction::East => { self.exploring_score },
          Direction::North | Direction::South => { self.exploring_score + 1000 },
          Direction::West => { self.exploring_score + 2000 },
        }
      },
      Direction::South => {
        match other_direction {
          Direction::South => { self.exploring_score },
          Direction::West | Direction::East => { self.exploring_score + 1000 },
          Direction::North => { self.exploring_score + 2000 },
        }

      },
      Direction::West => {
        match other_direction {
          Direction::West => { self.exploring_score },
          Direction::North | Direction::South => { self.exploring_score + 1000 },
          Direction::East => { self.exploring_score + 2000 },
        }
      },
    }
  }
}
