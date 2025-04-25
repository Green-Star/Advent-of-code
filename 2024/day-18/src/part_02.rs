use std::collections::VecDeque;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();

  let position = Position { x: 4, y: 5, score: 9 };
  let p = Position { x: 4, y: 5, score: 0 };
  assert!(position == p);

  let size = 70;
  let mut base_grid = Grid::new_empty(size + 1);

  let mut grid = base_grid.clone();
  println!("Computing initial path");
  let mut path = grid.explore();

  let mut final_result= "()".to_string();
  for s in data {
    println!("Falling ({s})");

    let wall = crate::core::parse_number_list_with_separator(&s, ",");
    let (y, x) = (wall[0], wall[1]);
    let position = Position { x, y, score: 0 };

    base_grid.add_wall(&position);

    if path.contains(&position) {
      println!("Path broken! Calling Dijkstra...");
      grid = base_grid.clone();
      path = grid.explore();
    }

    if path.is_empty() {
      final_result = s.clone();
      break;
    }
  }

  println!("Part 2 final result: {}", final_result);
}


#[derive(Debug, Copy, Clone)]
struct Position {
  x: usize,
  y: usize,
  score: i32,
}
impl PartialEq for Position {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Content {
  Wall,
}

#[derive(Debug, Clone, PartialEq)]
struct Tile {
  content: Option<Content>,
  exploring_score: Option<i32>,

  ancestors: Vec<Position>,
}

#[derive(Debug, Clone)]
struct Grid {
  map: Vec<Vec<Tile>>,
  ending_position: (usize, usize),

  yet_to_explore: VecDeque<Position>,
}
impl Grid {
  fn new_empty(grid_length: usize) -> Self {
    let mut map = vec![vec![Tile { content: None, exploring_score: None, ancestors: vec![] }; grid_length]; grid_length];
    let ending_position = (grid_length - 1, grid_length - 1);

    let mut to_explore = VecDeque::new();
    to_explore.push_back(Position { x: 0, y: 0, score: 0 });
    map[0][0].exploring_score = Some(0);

    Grid { map, ending_position, yet_to_explore: to_explore }
  }

  fn add_wall(&mut self, wall: &Position) {
    self.map[wall.x][wall.y] = Tile { content: Some(Content::Wall), exploring_score: None, ancestors: vec![] };
  }

  fn explore(&mut self) -> Vec<Position> {
    self.dijkstra(false)
  }

  fn dijkstra(&mut self, all_paths: bool) -> Vec<Position> {

    loop {

      self.yet_to_explore.make_contiguous().sort_by(|a, b| a.score.cmp(&b.score));
      let Some(vertice) = self.yet_to_explore.pop_front() else { break };

      let score = vertice.score + 1;

      let to_explore = self.get_neighbour_to_explore(&vertice);
      to_explore.iter().for_each(|&p| {
        if let Some(current_score) = self.map[p.x][p.y].exploring_score {
          if current_score < score { return }
          if current_score == score {
            let mut ancestors = self.map[p.x][p.y].ancestors.clone();
            /* We add ourselves to the ancestors of the next vertice, only if we need to find all the paths to the ending vertice */
            if all_paths {
              ancestors.push(Position { x: vertice.x, y: vertice.y, score });
            }
            self.map[p.x][p.y].ancestors = ancestors;
            return
          }
        }

        self.map[p.x][p.y].exploring_score = Some(score);
        self.map[p.x][p.y].ancestors = vec![ Position { x: vertice.x, y: vertice.y, score: vertice.score } ];
        self.yet_to_explore.push_back(p);
      });

      if let Some(_) = self.map[self.ending_position.0][self.ending_position.1].exploring_score { break }
    }

    self.build_return_path(&self.map[self.ending_position.0][self.ending_position.1])
  }

  fn build_return_path(&self, current_vertice: &Tile) -> Vec<Position> {
    if current_vertice.ancestors.is_empty() {return vec![] }

    let mut path = current_vertice.ancestors.clone();
    current_vertice.ancestors.iter().for_each(|v| path.append(&mut self.build_return_path(&self.map[v.x][v.y])));
    path
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

/*


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
  fn new_empty(grid_length: usize) -> Self {
    let map = vec![vec![Tile { content: None, exploring_score: None }; grid_length]; grid_length];
    let ending_position = (grid_length - 1, grid_length - 1);
    let mut to_explore = VecDeque::new();
    to_explore.push_back(Position { x: 0, y: 0, score: 0 });

    Grid { map, ending_position, yet_to_explore: to_explore }
  }

  fn add_wall(&mut self, position_string: String) {
    let mut split = position_string.split(",");
    let (y, x) = (split.next().unwrap().parse::<usize>().unwrap(), split.last().unwrap().parse::<usize>().unwrap());
    self.map[x][y] = Tile { content: Some(Content::Wall), exploring_score: None };
  }

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
}

*/