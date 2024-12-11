use std::collections::HashSet;

pub fn resolve(input_data_path: &str) {
  let data = crate::core::load_file_in_memory(input_data_path).unwrap();
  let mut hiking_map = transform_data(data);

  hiking_map.compute_hiking_trails();


  let trailheads = hiking_map.get_trailheads();
  let final_result = trailheads.iter().fold(0, |sum, t| sum + t.get_score());

  println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> HikingMap {
  let mut result = vec![];

  for i in 0..data.len() {
    let mut line = vec![];
    for c in data[i].chars() {
      line.push(Topographic { level: c.to_digit(10).unwrap_or(11), trails: HashSet::new() });
    }
    result.push(line);
  }

  HikingMap { map: result }
}


#[derive(Debug, Clone)]
struct Topographic {
  level: u32,
  trails: HashSet<(usize, usize)>,
}
impl Topographic {
  fn get_score(&self) -> usize {
    self.trails.len()
  }
}

#[derive(Debug, Clone)]
struct HikingMap {
  map: Vec<Vec<Topographic>>,
}
impl HikingMap {
  fn compute_hiking_trails(&mut self) {
    for i in 0..self.map.len() {
      for j in 0..self.map[i].len() {
        if self.map[i][j].level == 9 {
          self.follow_one_hiking_trail((i, j));
        }
      }
    }
  }
  fn follow_one_hiking_trail(&mut self, start_index: (usize, usize)) {
    self.map[start_index.0][start_index.1].trails.insert(start_index);

    /* Explore the 4 next indexes (North / West / East / South) */
    self.explore_one_hiking_trail((start_index.0.checked_add_signed(-1), start_index.1.checked_add_signed(0)), self.map[start_index.0][start_index.1].level, start_index);
    self.explore_one_hiking_trail((start_index.0.checked_add_signed(0), start_index.1.checked_add_signed(-1)), self.map[start_index.0][start_index.1].level, start_index);
    self.explore_one_hiking_trail((start_index.0.checked_add_signed(0), start_index.1.checked_add_signed(1)), self.map[start_index.0][start_index.1].level, start_index);
    self.explore_one_hiking_trail((start_index.0.checked_add_signed(1), start_index.1.checked_add_signed(0)), self.map[start_index.0][start_index.1].level, start_index);
  }

  fn explore_one_hiking_trail(&mut self, position: (Option<usize>, Option<usize>), previous_slope: u32, top_index: (usize, usize)) {
    let x;
    let y;

    /* Check if the current index to explore is inside the map (if not, stop here - there's nothing to explore) */
    match position {
      (Some(a), Some(b)) => {
        if a >= self.map.len() { return }
        if b >= self.map[a].len() { return }
        x = a;
        y = b;
      }
      _ => { return }
    }

    /* Check if we're still in the hiking trail (i.e. if the last explored index was 1 level above us) */
    let level = self.map[x][y].level;
    if level != (previous_slope - 1) { return }

    /* We're on the trail to `top_index`!, store it in our reachable trails */
    let have_not_been_explored = self.map[x][y].trails.insert(top_index);
    /* If we somehow were on another trail to `top_index`, let's just here -> there's nothing more to do on this tail as it was already explored */
    if have_not_been_explored == false { return }

    /* One quick check: if we're at ground level, there's nothing more to explore, so we'll stop here */
    if self.map[x][y].level == 0 { return }

    /* Otherwise, let's keep on following our trail path */
    /* (let's explore our 4 connected indexes) */
    self.explore_one_hiking_trail((x.checked_add_signed(-1), y.checked_add_signed(0)), level, top_index);
    self.explore_one_hiking_trail((x.checked_add_signed(0), y.checked_add_signed(-1)), level, top_index);
    self.explore_one_hiking_trail((x.checked_add_signed(0), y.checked_add_signed(1)), level, top_index);
    self.explore_one_hiking_trail((x.checked_add_signed(1), y.checked_add_signed(0)), level, top_index);
  }
  fn get_trailheads(&self) -> Vec<&Topographic> {
    let mut trailheads = vec![];

    for v in &self.map {
      for t in v {
        if t.level == 0 {
          trailheads.push(t);
        }
      }
    }

    trailheads
  }
}
