use std::collections::{HashMap, HashSet};

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let mut maze = transform_data(data);

  maze.race();

  let shortcuts = maze.find_shortcuts(20, 100);
  let shortcuts = shortcuts.iter().collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
  let final_result = shortcuts.len();

  println!("Part 2 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> Race {
  let mut map = vec![];
  let mut ending_position = Position { x: 0, y: 0 };
  let mut racer = Racer { position: ending_position, score: 0 };

  let mut x = 0;
  for s in data {
    let mut line = vec![];
    let mut y = 0;
    for c in s.chars() {
      let position = Position { x, y };
      match c {
        '#' => { line.push(Tile { position, content: Some(Content::Wall), racing_score: None }); },
        'E' => { line.push(Tile { position, content: None, racing_score: None }); ending_position = Position {x, y}; },
        'S' => { line.push(Tile { position, content: None, racing_score: None }); racer = Racer { position: position, score: 0 }; },
        _ => { line.push(Tile { position, content: None, racing_score: None }); }
      }
      y += 1;
    }
    map.push(line);
    x += 1;
  }

  Race { map, ending_position, racer }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Content {
  Wall,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
  x: usize,
  y: usize,
}
impl Position {
  fn distance_from(&self, other: Self) -> usize {
    other.x.abs_diff(self.x) + other.y.abs_diff(self.y)
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Tile {
  position: Position,
  content: Option<Content>,
  racing_score: Option<i64>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Racer {
  position: Position,
  score: i64,
}

#[derive(Debug, Clone)]
struct Race {
  map: Vec<Vec<Tile>>,
  ending_position: Position,
  racer: Racer,
}
impl Race {
  fn race(&mut self) {
    while self.racer.position != self.ending_position {
      self.map[self.racer.position.x][self.racer.position.y].racing_score = Some(self.racer.score);
      self.racer.position = self.get_next_tile().unwrap().position;
      self.racer.score += 1;
    }
    self.map[self.racer.position.x][self.racer.position.y].racing_score = Some(self.racer.score);
  }

  fn get_next_tile(&self) -> Option<Tile> {
    vec![(-1, 0), (1, 0), (0, -1), (0, 1)].iter().filter_map(|offset| {
      if let (Some(x), Some(y)) = (self.racer.position.x.checked_add_signed(offset.0), self.racer.position.y.checked_add_signed(offset.1)) {
        if self.check_next_tile_to_race(x, y) { return Some(self.map[x][y]) }
      }
      None
    }).take(1).last()
  }

  fn check_next_tile_to_race(&self, x: usize, y: usize) -> bool {
    self.map[x][y].content.is_none() && self.map[x][y].racing_score.is_none()
  }


  fn find_shortcuts(&self, shortcut_duration: usize, at_least: i64) -> Vec<Shortcut> {
    let mut tile_list = HashMap::new();
    self.map.iter()
            .for_each(|x| x.iter()
                                      .filter_map(|t| if let Some(_) = t.racing_score { Some(t) } else { None })
                                      .for_each(|t| { tile_list.insert(t.position, t.racing_score.unwrap()); })
                      );

    let shortcuts =
      tile_list.iter()
              .flat_map(|(origin, origin_score)| {
                    tile_list.iter()
                            .filter(|(position, _)| origin.distance_from(**position) <= shortcut_duration)
                            .filter(|(position, score)| at_least <= *score - *origin_score - (origin.distance_from(**position) as i64))
                            .map(|(position, score)| Shortcut { start_position: *origin, end_position: *position, score: *score - *origin_score - (origin.distance_from(*position) as i64) })
                            .collect::<Vec<_>>()
              })
              .collect::<Vec<_>>();
    shortcuts
  }

  fn _find_shortcut_from_tile(&self, position: Position, _shortcut_duration: usize, at_least: i64) -> Vec<Shortcut> {
    vec![(-1, 0), (1, 0), (0, -1), (0, 1)].iter().filter_map(|(offset_x, offset_y)| {
      let (Some(x), Some(y)) = (position.x.checked_add_signed(*offset_x), position.y.checked_add_signed(*offset_y)) else { return None };
      if x >= self.map.len() { return None }
      if y >= self.map[x].len() { return None }
      match self.map[x][y].content {
        None => { return None },
        Some(_) => {},
      }

      let (Some(x), Some(y)) = (x.checked_add_signed(*offset_x), y.checked_add_signed(*offset_y)) else { return None };
      if x >= self.map.len() { return None }
      if y >= self.map[x].len() { return None }
      match self.map[x][y].content {
        Some(_) => { return None },
        None => {
          let diff = self.map[x][y].racing_score.unwrap() - self.map[position.x][position.y].racing_score.unwrap() - 2;
          if at_least <= diff { return Some(Shortcut { start_position: position, end_position: Position { x, y }, score: diff }) }
        },
      }

      None
    }).collect()
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Shortcut {
  start_position: Position,
  end_position: Position,
  score: i64,
}

