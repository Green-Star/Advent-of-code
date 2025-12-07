use std::collections::{HashMap, HashSet};

pub fn resolve(s: &str) -> u64 {
    let mut manifold = transform_data(s);
    manifold.beam_simulation();
//    println!("{:?}", manifold);
    let final_result = manifold.ending_path.values().sum();
    final_result
}

fn transform_data(data: &str) -> Manifold {
    let mut splitters = HashSet::new();
    let mut beams = HashMap::new();

    for (y, l) in data.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                'S' => { beams.insert(Position { x, y }, 1); },
                '^' => { splitters.insert(Position { x, y }); },
                _ => {},
            }
        }
    }

    Manifold { splitters, beams, ending_path: HashMap::new(), height: data.lines().count(), length: data.lines().next().unwrap().len() }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Manifold {
    length: usize,
    height: usize,

    splitters: HashSet<Position>,
    beams: HashMap<Position, u64>,

    ending_path: HashMap<Position, u64>,
}
impl Manifold {
    fn beam_simulation(&mut self) {
        while self.beams.is_empty() == false {
            self.move_downward();
        }
    }

    fn move_downward(&mut self) {
        let mut next_beams = HashMap::new();

        for (&b, beam_count) in &self.beams {
//            println!("Move beam {:?}-{beam_count}", b);
            // Move the beam down
            let beam_down = Position { x: b.x, y: b.y + 1 };

            // Stop if the beam reached the end of the grid
            if beam_down.y > self.height {
                self.ending_path.entry(b).and_modify(|path_count| *path_count += beam_count).or_insert(*beam_count);
                continue
            }

            // Otherwise, split it we hit a splitter or go downward if there is nothing
            if let Some(_) = self.splitters.get(&beam_down) {
//                println!("Hit somethig at {:?}", beam_down);
                vec![(-1, 0), (1, 0)].iter().for_each(|&(offset_x, offset_y)| {
                    if let Some(next_x) = beam_down.x.checked_add_signed(offset_x) {
                        if let Some(next_y) = beam_down.y.checked_add_signed(offset_y) {
//
//                            println!("Next beam at ({next_x}, {next_y})...");
                            if next_x < self.length && next_y < self.height {
                                next_beams.entry(Position { x: next_x, y: next_y }).and_modify(|count| *count += beam_count).or_insert(*beam_count);
                            }
                        }
                    }
                });
            } else {
                next_beams.entry(beam_down).and_modify(|count| *count += beam_count).or_insert(*beam_count);
            }
        }

        self.beams = next_beams;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_02() {
        let test_input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

        assert_eq!(resolve(test_input), 40);
    }

    #[test]
    fn test_small_sample_01() {
        let test_input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
";

        assert_eq!(resolve(test_input), 4);
    }

    #[test]
    fn test_small_sample_02() {
        let test_input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
";

        assert_eq!(resolve(test_input), 8);
    }
}
