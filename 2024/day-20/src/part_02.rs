use std::collections::HashSet;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let mut maze = transform_data(data);

  maze.race();

  let shortcuts = maze.find_shortcuts(2, 2);
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


  fn find_shortcuts(&self, shortcut_duration: isize, at_least: i64) -> Vec<Shortcut> {
    let mut shortcuts = vec![];

    for x in 0..self.map.len() {
      for y in 0..self.map[x].len() {
        match self.map[x][y].content {
          Some(_) => continue,
          None => {},
        }
        let position = Position { x, y };
        shortcuts.append(&mut self.find_shortcut_from_tile(position, shortcut_duration, at_least));
      }
    }

    shortcuts
  }

  fn find_shortcut_from_tile(&self, position: Position, shortcut_duration: isize, at_least: i64) -> Vec<Shortcut> {
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

    /*
    let mut shortcuts = vec![];

    for offset_x in -shortcut_duration..=shortcut_duration {
      for offset_y in -shortcut_duration..=shortcut_duration {
        let duration = offset_x.abs() + offset_y.abs();
        if duration > shortcut_duration { continue }

        let Some(x) = position.x.checked_add_signed(offset_x) else { continue };
        let Some(y) = position.y.checked_add_signed(offset_y) else { continue };
        if x >= self.map.len() { continue }
        if y >= self.map[x].len() { continue }
        match self.map[x][y].content {
          Some(_) => continue,
          None => {},
        }

        let diff = (self.map[x][y].racing_score.unwrap()-1) - (self.map[position.x][position.y].racing_score.unwrap()-1);
        if at_least <= diff /*&& diff != (duration as i64)*/ {
          println!("({},{}) -> ({},{}) : {} to {} (diff {})", position.x, position.y, x, y, self.map[position.x][position.y].racing_score.unwrap(), self.map[x][y].racing_score.unwrap(), diff);
          shortcuts.push(Shortcut { start_position: position, end_position: Position { x, y }, score: diff })
        }
      }
    }

    shortcuts
    */
  }

  /*
  fn find_shortcut_from_tile(&self, position: Position, shortcut_duration: isize, at_least: i64) -> Vec<Shortcut> {
    let mut shortcuts = vec![];

    for offset_x in -shortcut_duration..=shortcut_duration {
      for offset_y in -shortcut_duration..=shortcut_duration {
        let duration = offset_x.abs() + offset_y.abs();
        if duration > shortcut_duration { continue }

        let Some(x) = position.x.checked_add_signed(offset_x) else { continue };
        let Some(y) = position.y.checked_add_signed(offset_y) else { continue };
        if x >= self.map.len() { continue }
        if y >= self.map[x].len() { continue }
        match self.map[x][y].content {
          Some(_) => continue,
          None => {},
        }

        let diff = (self.map[x][y].racing_score.unwrap()-1) - (self.map[position.x][position.y].racing_score.unwrap()-1);
        if at_least <= diff /*&& diff != (duration as i64)*/ {
          println!("({},{}) -> ({},{}) : {} to {} (diff {})", position.x, position.y, x, y, self.map[position.x][position.y].racing_score.unwrap(), self.map[x][y].racing_score.unwrap(), diff);
          shortcuts.push(Shortcut { start_position: position, end_position: Position { x, y }, score: diff })
        }
      }
    }

    shortcuts
  }
  */


  fn print(&self) {
    for x in 0..self.map.len() {
      for y in 0..self.map[x].len() {
        let position = Position { x, y };
        if self.racer.position == position {
          print!("0");
        } else if let Some(_) = self.map[x][y].content {
          print!("#");
        } else if let Some(_) = self.map[x][y].racing_score {
          print!("O");
        } else if position == self.ending_position {
          print!("E");
        } else {
          print!(".");
        }
      }
      println!();
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Shortcut {
  start_position: Position,
  end_position: Position,
  score: i64,
}

