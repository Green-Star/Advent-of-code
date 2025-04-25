use std::collections::VecDeque;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();

  let size = 70;
  let mut grid = transform_data(data, 1024, size+1);

  let result = grid.explore();
  grid.print_explored();
  let final_result = result.unwrap();

  println!("Part 1 final result: {}", final_result);
}

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
