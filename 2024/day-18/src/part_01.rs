use std::collections::VecDeque;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();

  let size = 6;
  let mut grid = transform_data(data, 12, size+1);

  grid.print();
  println!("*****");
  let result = grid.explore();
  grid.print_explored();

  println!("{:?}", result);
  /*

  let searched = a_star_search(&maze.map, &Location { x: 0, y: 0, score: 0, heuristic: (size*size) as i32},&Location { x: size, y: size, score: (size*size) as i32, heuristic: 0 });
  if searched.is_err() { println!("Not found!"); return; };
  let searched = searched.unwrap();
  print_a_start_searched_maze(&maze.map, &searched);
  let finished = searched.iter().find(|l| l.x == size && l.y == size).unwrap();
  println!("Shortest path: {}", finished.score);

  */
  let final_result = result.unwrap();

  println!("Part 1 final result: {}", final_result);
}
/*
fn transform_data(data: Vec<String>, limit: i32, grid_length: usize) -> Maze {
  let mut map = vec![vec![Tile { content: None, exploring_score: None }; grid_length]; grid_length];
  let ending_position = (grid_length - 1, grid_length - 1);
  let mut explorer = VecDeque::new();
  explorer.push_back(Explorer { position: (0, 0), direction: Direction::East, exploring_score: 0 });

  let mut i = 1;
  for s in data {
    let mut split = s.split(",");
    let (y, x) = (split.next().unwrap().parse::<usize>().unwrap(), split.last().unwrap().parse::<usize>().unwrap());
    map[x][y] = Tile { content: Some(Content::Wall), exploring_score: None };
    if i >= limit { break }
    i += 1;
  }

  Maze { map, ending_position, yet_to_explore: explorer }
}
  */


fn transform_data(data: Vec<String>, limit: i32, grid_length: usize) -> Grid {
  let mut map = vec![vec![Tile { content: None, exploring_score: None }; grid_length]; grid_length];
  let ending_position = (grid_length - 1, grid_length - 1);
  let mut to_explore = VecDeque::new();
  to_explore.push_back(Position { x: 0, y: 0, score: 0 });

  let mut i = 1;
  for s in data {
    let mut split = s.split(",");
    let (y, x) = (split.next().unwrap().parse::<usize>().unwrap(), split.last().unwrap().parse::<usize>().unwrap());
    map[x][y] = Tile { content: Some(Content::Wall), exploring_score: None };
    if i >= limit { break }
    i += 1;
  }

  Grid { map, ending_position, yet_to_explore: to_explore }
}


#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
  x: usize,
  y: usize,
  score: i32,
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
struct Grid {
  map: Vec<Vec<Tile>>,
  ending_position: (usize, usize),

  yet_to_explore: VecDeque<Position>,
}
impl Grid {
  fn explore(&mut self) -> Option<i32> {
    loop {

      if let Some(final_score) = self.map[self.ending_position.0][self.ending_position.1].exploring_score {
        return Some(final_score)
      }

      self.yet_to_explore.make_contiguous().sort_by(|a, b| a.score.cmp(&b.score));
      let Some(vertice) = self.yet_to_explore.pop_front() else {
        return None
      };

      match self.map[vertice.x][vertice.y].exploring_score {
        Some(score) => if vertice.score >= score { continue; },
        None => {},
      }
      self.map[vertice.x][vertice.y].exploring_score = Some(vertice.score);

      let to_explore = self.get_neighbour_to_explore(&vertice);
      println!("{:?}", to_explore);
      to_explore.iter().for_each(|&p| self.yet_to_explore.push_back(p));

    }
  }

  fn get_neighbour_to_explore(&self, position: &Position) -> Vec<Position> {
    let mut neighbours = vec![];
    let score = position.score + 1;

    let y = position.y;
    if let Some(x) = position.x.checked_add_signed(-1) {
      match self.map[x][y].content {
        None => neighbours.push(Position { x, y, score }),
        _ => {},
      }
    }

    let x = position.x + 1;
    let y = position.y;
    if x < self.map.len() {
      match self.map[x][y].content {
        None => neighbours.push(Position { x, y, score }),
        _ => {},
      }
    }

    let x = position.x;
    if let Some(y) = position.y.checked_add_signed(-1) {
      match self.map[x][y].content {
        None => neighbours.push(Position { x, y, score }),
        _ => {},
      }
    }

    let x = position.x;
    let y = position.y + 1;
    if y < self.map[x].len() {
      match self.map[x][y].content {
        None => neighbours.push(Position { x, y, score }),
        _ => {},
      }
    }

    neighbours
  }

  fn print(&self) {
    for i in 0..self.map.len() {
      for j in 0..self.map[i].len() {
        match self.map[i][j].content {
          Some(Content::Wall) => print!("#"),
          None => print!("."),
        }
      }
      println!("");
    }
  }
  fn print_explored(&self) {
    for i in 0..self.map.len() {
      for j in 0..self.map[i].len() {
        match self.map[i][j].content {
          Some(Content::Wall) => print!("#"),
          None => { if let Some(_) = self.map[i][j].exploring_score { print!("O") } else { print!(".") } },
        }
      }
      println!("");
    }
  }
}









#[derive(Debug, Copy, Clone, PartialEq)]
struct Location {
  x: usize,
  y: usize,
  score: i32,
  heuristic: i32,
}

fn complete_a_star_search(map: &Vec<Vec<Tile>>, start_location: &Location, end_location: &Location) -> Result<Vec<Location>, ()> {
  let mut closed_list = vec![];
  let mut open_list = VecDeque::new();
  let mut final_score = None;

  open_list.push_back(*start_location);
  loop {
    let location;
    open_list.make_contiguous().sort_by(|a, b| a.heuristic.cmp(&b.heuristic));
    println!("Pending queue: {:?}", open_list);
    match open_list.pop_front() {
      None => break,
      Some(l) => location = l,
    }

    println!("Exploring ({}, {})...", location.x, location.y);

    if location.x == end_location.x && location.y == end_location.y {
      closed_list.push(location);
      final_score = Some(location.score);
//      return Ok(closed_list);
    }

    if let Some(score) = final_score {
      if location.score >= score { continue }
    }


    if let Some((north_x, north_y)) = get_neighbour_index(map, location, Direction::North) {
      let neighbour = Location { x: north_x, y: north_y, score: location.score + 1, heuristic:  distance_to(&(north_x, north_y), &(end_location.x, end_location.y)) };

      if map[neighbour.x][neighbour.y].content == None && closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) == false {
        match open_list.iter().find(|l| l.x == neighbour.x && l.y == neighbour.y) {
          None => {
            println!("Add North ({}, {})", neighbour.x, neighbour.y);
            open_list.push_back(neighbour);
          },
          Some(v) => {
            if v.score > neighbour.score {
              open_list.retain(|l| l.x != neighbour.x || l.y != neighbour.y);
              println!("Replace North ({}, {})", neighbour.x, neighbour.y);
              open_list.push_back(neighbour);
            } else if v.score == neighbour.score {

            }
          }
        }
      }
      /*
      let neighbour = Location { x: north_x, y: north_y, score: location.score, heuristic: location.heuristic };
      if map[neighbour.x][neighbour.y].content == None &&
        !(closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) ||
          open_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y && l.score < neighbour.score))
      {
        let v = Location {
          x: north_x,
          y: north_y,
          score: location.score + 1,
          heuristic: distance_to(&(north_x, north_y), &(end_location.x, end_location.y)),
        };
        println!("Add North ({}, {})", v.x, v.y);
        open_list.push_back(v.clone());
      }
      */
    }
    if let Some((east_x, east_y)) = get_neighbour_index(map, location, Direction::East) {
      /*
      let neighbour = Location { x: east_x, y: east_y, score: location.score, heuristic: location.heuristic };
      if map[neighbour.x][neighbour.y].content == None &&
        !(closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) ||
          open_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y && l.score < neighbour.score))
      {
        let v = Location {
          x: east_x,
          y: east_y,
          score: location.score + 1,
          heuristic: distance_to(&(east_x, east_y), &(end_location.x, end_location.y)),
        };
        println!("Add East ({}, {})", v.x, v.y);
        open_list.push_back(v.clone());
      }
      */
      let neighbour = Location { x: east_x, y: east_y, score: location.score + 1, heuristic:  distance_to(&(east_x, east_y), &(end_location.x, end_location.y)) };

      if map[neighbour.x][neighbour.y].content == None && closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) == false {
        match open_list.iter().find(|l| l.x == neighbour.x && l.y == neighbour.y) {
          None => {
            println!("Add East ({}, {})", neighbour.x, neighbour.y);
            open_list.push_back(neighbour);
          },
          Some(v) => {
            if v.score > neighbour.score {
              open_list.retain(|l| l.x != neighbour.x || l.y != neighbour.y);
              println!("Replace East ({}, {})", neighbour.x, neighbour.y);
              open_list.push_back(neighbour);
            } else if v.score == neighbour.score {

            }
          }
        }
      }
    }
    if let Some((south_x, south_y)) = get_neighbour_index(map, location, Direction::South) {
      /*
      let neighbour = Location { x: south_x, y: south_y, score: location.score, heuristic: location.heuristic };
      if map[neighbour.x][neighbour.y].content == None &&
        !(closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) ||
          open_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y && l.score < neighbour.score))
      {
        let v = Location {
          x: south_x,
          y: south_y,
          score: location.score + 1,
          heuristic: distance_to(&(south_x, south_y), &(end_location.x, end_location.y)),
        };
        println!("Add South ({}, {})", v.x, v.y);
        open_list.push_back(v.clone());
      }
      */
      let neighbour = Location { x: south_x, y: south_y, score: location.score + 1, heuristic:  distance_to(&(south_x, south_y), &(end_location.x, end_location.y)) };

      if map[neighbour.x][neighbour.y].content == None && closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) == false {
        match open_list.iter().find(|l| l.x == neighbour.x && l.y == neighbour.y) {
          None => {
            println!("Add South ({}, {})", neighbour.x, neighbour.y);
            open_list.push_back(neighbour);
          },
          Some(v) => {
            if v.score > neighbour.score {
              open_list.retain(|l| l.x != neighbour.x || l.y != neighbour.y);
              println!("Replace South ({}, {})", neighbour.x, neighbour.y);
              open_list.push_back(neighbour);
            } else if v.score == neighbour.score {

            }
          }
        }
      }
    }
    if let Some((west_x, west_y)) = get_neighbour_index(map, location, Direction::West) {
      /*
      let neighbour = Location { x: west_x, y: west_y, score: location.score, heuristic: location.heuristic };
      if map[neighbour.x][neighbour.y].content == None &&
        !(closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) ||
          open_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y && l.score < neighbour.score))
      {
        let v = Location {
          x: west_x,
          y: west_y,
          score: location.score + 1,
          heuristic: distance_to(&(west_x, west_y), &(end_location.x, end_location.y)),
        };
        println!("Add West ({}, {})", v.x, v.y);
        open_list.push_back(v.clone());
      }
      */
      let neighbour = Location { x: west_x, y: west_y, score: location.score + 1, heuristic:  distance_to(&(west_x, west_y), &(end_location.x, end_location.y)) };

      if map[neighbour.x][neighbour.y].content == None && closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) == false {
        match open_list.iter().find(|l| l.x == neighbour.x && l.y == neighbour.y) {
          None => {
            println!("Add West ({}, {})", neighbour.x, neighbour.y);
            open_list.push_back(neighbour);
          },
          Some(v) => {
            if v.score > neighbour.score {
              open_list.retain(|l| l.x != neighbour.x || l.y != neighbour.y);
              println!("Replace West ({}, {})", neighbour.x, neighbour.y);
              open_list.push_back(neighbour);
            } else if v.score == neighbour.score {

            }
          }
        }
      }
    }

    closed_list.push(location);

    println!("*****");
  }

  match final_score {
    None => Err(()),
    Some(score) => Ok(closed_list),
  }
}



fn a_star_search(map: &Vec<Vec<Tile>>, start_location: &Location, end_location: &Location) -> Result<Vec<Location>, ()> {
  let mut closed_list = vec![];
  let mut open_list = VecDeque::new();

  open_list.push_back(*start_location);
  loop {
    let location;
    open_list.make_contiguous().sort_by(|a, b| a.heuristic.cmp(&b.heuristic));
    println!("Pending queue: {:?}", open_list);
    match open_list.pop_front() {
      None => break,
      Some(l) => location = l,
    }

    println!("Exploring ({}, {})...", location.x, location.y);

    if location.x == end_location.x && location.y == end_location.y {
      closed_list.push(location);
      return Ok(closed_list);
    }

    if let Some((north_x, north_y)) = get_neighbour_index(map, location, Direction::North) {
      let neighbour = Location { x: north_x, y: north_y, score: location.score + 1, heuristic:  distance_to(&(north_x, north_y), &(end_location.x, end_location.y)) };

      if map[neighbour.x][neighbour.y].content == None && closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) == false {
        match open_list.iter().find(|l| l.x == neighbour.x && l.y == neighbour.y) {
          None => {
            println!("Add North ({}, {})", neighbour.x, neighbour.y);
            open_list.push_back(neighbour);
          },
          Some(v) => {
            if v.score > neighbour.score {
              open_list.retain(|l| l.x != neighbour.x || l.y != neighbour.y);
              println!("Replace North ({}, {})", neighbour.x, neighbour.y);
              open_list.push_back(neighbour);
            } else if v.score == neighbour.score {

            }
          }
        }
      }
      /*
      let neighbour = Location { x: north_x, y: north_y, score: location.score, heuristic: location.heuristic };
      if map[neighbour.x][neighbour.y].content == None &&
        !(closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) ||
          open_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y && l.score < neighbour.score))
      {
        let v = Location {
          x: north_x,
          y: north_y,
          score: location.score + 1,
          heuristic: distance_to(&(north_x, north_y), &(end_location.x, end_location.y)),
        };
        println!("Add North ({}, {})", v.x, v.y);
        open_list.push_back(v.clone());
      }
      */
    }
    if let Some((east_x, east_y)) = get_neighbour_index(map, location, Direction::East) {
      /*
      let neighbour = Location { x: east_x, y: east_y, score: location.score, heuristic: location.heuristic };
      if map[neighbour.x][neighbour.y].content == None &&
        !(closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) ||
          open_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y && l.score < neighbour.score))
      {
        let v = Location {
          x: east_x,
          y: east_y,
          score: location.score + 1,
          heuristic: distance_to(&(east_x, east_y), &(end_location.x, end_location.y)),
        };
        println!("Add East ({}, {})", v.x, v.y);
        open_list.push_back(v.clone());
      }
      */
      let neighbour = Location { x: east_x, y: east_y, score: location.score + 1, heuristic:  distance_to(&(east_x, east_y), &(end_location.x, end_location.y)) };

      if map[neighbour.x][neighbour.y].content == None && closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) == false {
        match open_list.iter().find(|l| l.x == neighbour.x && l.y == neighbour.y) {
          None => {
            println!("Add East ({}, {})", neighbour.x, neighbour.y);
            open_list.push_back(neighbour);
          },
          Some(v) => {
            if v.score > neighbour.score {
              open_list.retain(|l| l.x != neighbour.x || l.y != neighbour.y);
              println!("Replace East ({}, {})", neighbour.x, neighbour.y);
              open_list.push_back(neighbour);
            } else if v.score == neighbour.score {

            }
          }
        }
      }
    }
    if let Some((south_x, south_y)) = get_neighbour_index(map, location, Direction::South) {
      /*
      let neighbour = Location { x: south_x, y: south_y, score: location.score, heuristic: location.heuristic };
      if map[neighbour.x][neighbour.y].content == None &&
        !(closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) ||
          open_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y && l.score < neighbour.score))
      {
        let v = Location {
          x: south_x,
          y: south_y,
          score: location.score + 1,
          heuristic: distance_to(&(south_x, south_y), &(end_location.x, end_location.y)),
        };
        println!("Add South ({}, {})", v.x, v.y);
        open_list.push_back(v.clone());
      }
      */
      let neighbour = Location { x: south_x, y: south_y, score: location.score + 1, heuristic:  distance_to(&(south_x, south_y), &(end_location.x, end_location.y)) };

      if map[neighbour.x][neighbour.y].content == None && closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) == false {
        match open_list.iter().find(|l| l.x == neighbour.x && l.y == neighbour.y) {
          None => {
            println!("Add South ({}, {})", neighbour.x, neighbour.y);
            open_list.push_back(neighbour);
          },
          Some(v) => {
            if v.score > neighbour.score {
              open_list.retain(|l| l.x != neighbour.x || l.y != neighbour.y);
              println!("Replace South ({}, {})", neighbour.x, neighbour.y);
              open_list.push_back(neighbour);
            } else if v.score == neighbour.score {

            }
          }
        }
      }
    }
    if let Some((west_x, west_y)) = get_neighbour_index(map, location, Direction::West) {
      /*
      let neighbour = Location { x: west_x, y: west_y, score: location.score, heuristic: location.heuristic };
      if map[neighbour.x][neighbour.y].content == None &&
        !(closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) ||
          open_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y && l.score < neighbour.score))
      {
        let v = Location {
          x: west_x,
          y: west_y,
          score: location.score + 1,
          heuristic: distance_to(&(west_x, west_y), &(end_location.x, end_location.y)),
        };
        println!("Add West ({}, {})", v.x, v.y);
        open_list.push_back(v.clone());
      }
      */
      let neighbour = Location { x: west_x, y: west_y, score: location.score + 1, heuristic:  distance_to(&(west_x, west_y), &(end_location.x, end_location.y)) };

      if map[neighbour.x][neighbour.y].content == None && closed_list.iter().any(|l| l.x == neighbour.x && l.y == neighbour.y) == false {
        match open_list.iter().find(|l| l.x == neighbour.x && l.y == neighbour.y) {
          None => {
            println!("Add West ({}, {})", neighbour.x, neighbour.y);
            open_list.push_back(neighbour);
          },
          Some(v) => {
            if v.score > neighbour.score {
              open_list.retain(|l| l.x != neighbour.x || l.y != neighbour.y);
              println!("Replace West ({}, {})", neighbour.x, neighbour.y);
              open_list.push_back(neighbour);
            } else if v.score == neighbour.score {

            }
          }
        }
      }
    }

    closed_list.push(location);

    println!("*****");
  }

  Err(())
}

fn get_neighbour_index(map: &Vec<Vec<Tile>>, location: Location, direction: Direction) -> Option<(usize, usize)> {
  let (offset_x, offset_y) = direction.offset();
  match (location.x.checked_add_signed(offset_x), location.y.checked_add_signed(offset_y)) {
    (Some(x), Some(y)) => {
      if x >= map.len() { return None }
      if y >= map[x].len() { return None }

      Some((x, y))
    },
    _ => None,
  }
}

fn print_a_start_searched_maze(map: &Vec<Vec<Tile>>, searched: &Vec<Location>) {
  for i in 0..map.len() {
    for j in 0..map[i].len() {
      if searched.iter().any(|l| l.x == i && l.y == j) {
        print!("X");
      } else if map[i][j].content == Some(Content::Wall) {
        print!("#");
      } else {
        print!(".");
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


#[derive(Debug, Clone)]
struct Maze {
  map: Vec<Vec<Tile>>,
  ending_position: (usize, usize),

  yet_to_explore: VecDeque<Explorer>,
}
impl Maze {
  /*
  fn a_star(&mut self) {
    let e = Explorer { position: (0, 0), direction: Direction::East, exploring_score: 0 };

    let mut open_set = VecDeque::new();
    open_set.push_back(e);

    let came_from = HashSet::new();

    self.map[e.position.0][e.position.1].g_score = Some(0);
    self.map[e.position.0][e.position.1].f_score = Some(distance_to(&e.position, &self.ending_position));

    loop {
      open_set.make_contiguous().sort_by(|a, b| a.exploring_score.cmp(&b.exploring_score));
      match self.yet_to_explore.pop_front() {
        /* Grab the closest from start yet-to-explore path (filled with the starting tile at beginning) and explore it straight ahead */
        Some(e) => self.explore_path(e),
        /* If there isn't any path to explore, we're finished */
        None => break,
      }
    }

  }

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
  }
  */
}

fn distance_to(pos: &(usize, usize), ending_position: &(usize, usize)) -> i32 {
  ((ending_position.0 - pos.0) + (ending_position.1 - pos.1)) as i32
}


fn print_maze(maze: &Maze) {
  for i in 0..maze.map.len() {
    for j in 0..maze.map[i].len() {
      match maze.map[i][j].content {
        Some(Content::Wall) => print!("#"),
        None => print!("."),
      }
    }
    println!("");
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
