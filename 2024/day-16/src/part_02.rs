use std::{collections::{HashSet, VecDeque}, hash::{Hash, Hasher}};

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let mut maze = transform_data(data);

  maze.explore();
  print_explored_maze(&maze);
  maze.find_best_path();
  let mut race =  maze.prune_best_path();
  race.explore_all_best_paths();
  //  println!("{:?}", r);
  //  println!("*****");
  pretty_print_explored_maze(&maze);
  print_best_path(&race);
  let final_result: usize = race.map.iter().map(|v| v.iter().filter(|t| t.is_best).collect::<Vec<&Tile>>()).map(|fv| fv.len()).sum();

//  let final_result = r.len();

  println!("Part 2 final result: {} (521 < [538] < 560)", final_result);
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
        '#' => { line.push(Tile { content: Some(Content::Wall), exploring_score: None, is_best: false, neighbours: HashSet::new(), next_tile_on_best_path: None }); },
        'E' => { line.push(Tile { content: None, exploring_score: None, is_best: true, neighbours: HashSet::new(), next_tile_on_best_path: None }); ending_position = (x, y); }, // Note: The ending tile is on the best path by default
        'S' => { line.push(Tile { content: None, exploring_score: Some((0, Direction::East)), is_best: false, neighbours: HashSet::new(), next_tile_on_best_path: None }); explorer.push_back(Explorer { position: (x, y), direction: Direction::East, exploring_score: 0 }); },
        _ => { line.push(Tile { content: None, exploring_score: None, is_best: false, neighbours: HashSet::new(), next_tile_on_best_path: None }); }
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Content {
  Wall,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
  content: Option<Content>,
  exploring_score: Option<(i32, Direction)>,
  is_best: bool,

  neighbours: HashSet<(usize, usize)>,
  next_tile_on_best_path: Option<(usize, usize)>
}
impl Hash for Tile {
  fn hash<H: Hasher> (&self, state: &mut H) {
    self.content.hash(state);
    self.exploring_score.hash(state);
    self.is_best.hash(state);
  }
}
impl Tile {
  fn get_leveled_score(&self, other_explorer: &Explorer) -> i32 {
    let (score, direction) = self.exploring_score.unwrap();

    match direction {
      Direction::North => {
        match other_explorer.direction {
          Direction::North => { score },
          Direction::East | Direction::West => { score + 1000 },
          Direction::South => { score },
        }
      },
      Direction::East => {
        match other_explorer.direction {
          Direction::East => { score },
          Direction::North | Direction::South => { score + 1000 },
          Direction::West => { score },
        }
      },
      Direction::South => {
        match other_explorer.direction {
          Direction::South => { score },
          Direction::West | Direction::East => { score + 1000 },
          Direction::North => { score },
        }

      },
      Direction::West => {
        match other_explorer.direction {
          Direction::West => { score },
          Direction::North | Direction::South => { score + 1000 },
          Direction::East => { score + 0 },
        }
      },
    }
  }
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
    let mut i = 0;
    /* I will do sort of a Dijkstra algorithm here */
    loop {
      self.yet_to_explore.make_contiguous().sort_by(|a, b| a.exploring_score.cmp(&b.exploring_score));
      match self.yet_to_explore.pop_front() {
        /* Grab the closest from start yet-to-explore path (filled with the starting tile at beginning) and explore it straight ahead */
        Some(e) => self.explore_path(e),
        /* If there isn't any path to explore, we're finished */
        None => break,
      }
//      println!("{} explorers", self.yet_to_explore.len());
    }
  }


  /* Explore one path, straight ahead, recording all connected paths to it */
  fn explore_path(&mut self, e: Explorer) {
    //    println!("Explorer: {:?}", e);
    //    println!("{:?}", self.explorer_done);

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
      /* Not explored yet */
      /*  => Simply record ourself as the closest neighbour of the tile and set its score */
      None => {
        self.map[next.position.0][next.position.1].exploring_score = Some((next.exploring_score, next.direction));
        self.map[next.position.0][next.position.1].neighbours = HashSet::new();
        self.map[next.position.0][next.position.1].neighbours.insert(e.position);
      },
      /* Already explored */
      /* We do a simple path search here, so we will keep only one neighbour for each tile */
      Some((score, direction)) => {
        let leveled_score = self.map[next.position.0][next.position.1].get_leveled_score(&e);

        /* If we're above (or equal) the shortest score, then there is another shortest path to this tile and we stop here */
        if leveled_score <= next.exploring_score { return }
        /* If we're below the shortest score, we found a shortest path leading to this tile */
          /* => Update the score of this tile and record ourself as the only closest neighbour */
        if leveled_score > next.exploring_score {
          self.map[next.position.0][next.position.1].exploring_score = Some((next.exploring_score, next.direction));
          self.map[next.position.0][next.position.1].neighbours = HashSet::new();
          self.map[next.position.0][next.position.1].neighbours.insert(e.position);
        }
      },
    }

    /* Small optimization, if we're already farther than the end tile, stop here */
    if let Some((final_score, _)) = self.map[self.ending_position.0][self.ending_position.1].exploring_score {
      if next.exploring_score > final_score { return }
    }

    /* Then, let's keep exploring this path (until we reach a wall) */
    self.explore_path(next);
  }



  fn explore_all_best_paths(&mut self) {
    let mut i = 0;
    /* I will do sort of a Dijkstra algorithm here */
    loop {
      self.yet_to_explore.make_contiguous().sort_by(|a, b| a.exploring_score.cmp(&b.exploring_score));
      match self.yet_to_explore.pop_front() {
        /* Grab the closest from start yet-to-explore path (filled with the starting tile at beginning) and explore it straight ahead */
        Some(e) => self.find_all_best_paths(e),
        /* If there isn't any path to explore, we're finished */
        None => break,
      }
    }
  }

  fn find_all_best_paths(&mut self, e: Explorer) {

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

    /* If we're exploring the best path, simply push new explorers (do nothing on the already-record best path) */
    if self.map[e.position.0][e.position.1].is_best {
      return self.find_all_best_paths(next);
    }


    /* We're on an accessible tile (which is not on the best path), let's check its score */
    match self.map[next.position.0][next.position.1].exploring_score {
      /* No score: the tile wasn't explored yet, */
      /*  => Simply record ourself as the closest neighbour of the tile and set its score */
      None => {
        self.map[next.position.0][next.position.1].exploring_score = Some((next.exploring_score, next.direction));
        self.map[next.position.0][next.position.1].neighbours = HashSet::new();
        self.map[next.position.0][next.position.1].neighbours.insert(e.position);
      },
      /* Already explored */
      Some((score, direction)) => {
        /* Let's check this tile:  */
        if self.map[next.position.0][next.position.1].is_best {
          /* We found a best-path tile:  */
          println!("Colliding with best path at ({:?})", next);

          /* Compute exploring score of this tile */
          let leveled_score = self.map[next.position.0][next.position.1].get_leveled_score(&e);

          /* If we're above the shortest score, then there is another shortest path to this tile and we stop here */
          if leveled_score < next.exploring_score { return }
          /* If we equal the shortest score, check if we're back on the best path (check the distance with the nex best-path tile) */
          if leveled_score == next.exploring_score {
            /* Being optimisitic here: admit there isn't two best paths ending on the end vertice */
            let next_best_path = self.map[next.position.0][next.position.1].next_tile_on_best_path.unwrap();
            let Some((next_best_score, next_best_direction)) = self.map[next_best_path.0][next_best_path.1].exploring_score else { return };

            /*** Il faut regarder s'il y a un changement de direction entre le next et le best path ***/
            println!("Checking {:?} != {:?} && {:?} == {:?} && {:?} == {:?}",
            next_best_direction, direction, e.direction, next_direction, next_best_score, e.exploring_score + 1 + 1);
            if next_best_direction != direction && e.direction == next_direction && next_best_score == e.exploring_score + 1 + 1 {
              println!("True");
              self.map[next_position.0][next_position.1].neighbours.insert(e.position);
              self.backtrack_best_path(&e.position);
            }


              /*
            println!("Checking {:?} with {next_best_score}&{:?}", e, direction);
              let next_best_score = self.map[next_best_path.0][next_best_path.1].get_leveled_score(&e);
              //if next_best_score == e.exploring_score + 1 + 1 {
              if e.direction == direction && e.exploring_score + 1 + 1 == next_best_score {
            println!("Found new best path");
                self.map[next_position.0][next_position.1].neighbours.insert(e.position);
                self.backtrack_best_path(&e.position);
              }
              */
          }
          /* If we're below the shortest score, it's a non-sense */
          //panic!();
        } else {
          /* Compute exploring score of this tile */
          let leveled_score = self.map[next.position.0][next.position.1].get_leveled_score(&e);

          /* If we're above the shortest score, then there is another shortest path to this tile and we stop here */
          if leveled_score < next.exploring_score { return }
          /* If we equal the shortest score, then have to record ourself as one the closest neighbour of the tile */
          if leveled_score == next.exploring_score {
            /* Yet another check: if the other is going straighforward, we're are on a useless turn, so dont't record ourself */
            let next_step_short_path = &self.map[next.position.0.checked_add_signed(direction.offset().0).unwrap()][next.position.1.checked_add_signed(direction.offset().1).unwrap()];

            self.map[next_position.0][next_position.1].neighbours.insert(e.position);
          }
          /* If we're below the shortest score, we found a shortest path leading to this tile */
            /* => Update the score of this tile and record ourself as the only closest neighbour */
          if leveled_score > next.exploring_score {
            self.map[next.position.0][next.position.1].exploring_score = Some((next.exploring_score, next.direction));
            self.map[next.position.0][next.position.1].neighbours = HashSet::new();
            self.map[next.position.0][next.position.1].neighbours.insert(e.position);
          }
        }
      },
    }

    /* Small optimization, if we're already farther than the end tile, stop here */
    if let Some((final_score, _)) = self.map[self.ending_position.0][self.ending_position.1].exploring_score {
      if next.exploring_score > final_score { return }
    }

    /* Then, let's keep exploring this path (until we reach a wall) */
    self.find_all_best_paths(next);
  }

fn find_best_path(&mut self) {
  /* Good thing, we did a Dijkstra algorithm to find the shortest path between starting and ending points */
  /* Thus, we already know the best path(s): simply start from the ending position and reverse the path (following decreasing exploring_score) until we reach the starting point (which has exploring score 0) */
  let index = (self.ending_position.0, self.ending_position.1);
  // TODO

  self.follow_best_path(&index);
}
fn follow_best_path(&mut self, index: &(usize, usize)) {
  println!("Best path: ({}, {})", index.0, index.1);

  self.map[index.0][index.1].is_best = true;
  let next = self.map[index.0][index.1].neighbours.clone();

  for p in next {
    self.map[p.0][p.1].next_tile_on_best_path = Some(*index);
    self.follow_best_path(&p);
  }
}
fn backtrack_best_path(&mut self, index: &(usize, usize)) {
  if self.map[index.0][index.1].is_best { return }

  self.map[index.0][index.1].is_best = true;

  let next = self.map[index.0][index.1].neighbours.clone();
  for p in next {
    self.map[p.0][p.1].next_tile_on_best_path = Some(*index);
    self.backtrack_best_path(&p);
  }
}

fn prune_best_path(&self) -> Self {
  let mut new = self.clone();

  new.yet_to_explore = VecDeque::new();
  new.explorer_done = HashSet::new();

  for i in 0..new.map.len() {
    for j in 0..new.map[i].len() {
      if new.map[i][j].is_best == false {
        new.map[i][j].exploring_score = None;
        new.map[i][j].neighbours = HashSet::new();
      } else {
        if let Some((score, _)) = new.map[i][j].exploring_score {
          if score == 0 {
            new.yet_to_explore.push_back(Explorer { position: (i, j), direction: Direction::East, exploring_score: 0 })
          }
        }
      }
    }
  }

  new
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
        Some(Content::Wall) => print!("[####]"),
        None => {
          match maze.map[i][j].exploring_score {
            Some((score, _)) => print!("[{:04}]", score),
            None => print!("[    ]"),
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
/*

  println!("{:?}", maze.map[7][7]);
  println!("{:?}", maze.map[7][6]);
  println!("{:?}", maze.map[7][5]);
  println!("{:?}", maze.map[8][5]);
  */
}

fn pretty_print_explored_maze(maze: &Maze) {
  for i in 0..maze.map.len() {
    for j in 0..maze.map[i].len() {
      match maze.map[i][j].content {
        Some(Content::Wall) => print!("[#####]"),
        None => {
          if maze.map[i][j].is_best {
            print!("[{:05}]", maze.map[i][j].exploring_score.unwrap().0);
          } else {
            print!("[     ]")
          }
        }
      }
    }
    println!("");
  }
}

#[derive(Debug, Copy, Clone, Eq)]
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
impl Hash for Explorer {
  fn hash<H: Hasher> (&self, state: &mut H) {
    self.position.hash(state);
    self.direction.hash(state);
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
