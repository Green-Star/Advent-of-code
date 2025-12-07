use std::collections::{HashMap, HashSet};

pub fn resolve(s: &str) -> usize {
    let mut manifold = transform_data(s);
    manifold.beam_simulation();
    let final_result = manifold.splitters.iter().filter(|(_, touched)| **touched).count();
    final_result
}

fn transform_data(data: &str) -> Manifold {
    let mut splitters = HashMap::new();
    let mut beams = HashSet::new();

    for (y, l) in data.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                'S' => { beams.insert(Position { x, y }); },
                '^' => { splitters.insert(Position { x, y }, false); },
                _ => {},
            }
        }
    }

    Manifold { splitters, beams, height: data.lines().count(), length: data.lines().next().unwrap().len() }
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

    splitters: HashMap<Position, bool>,
    beams: HashSet<Position>,
}
impl Manifold {
    fn beam_simulation(&mut self) {
        while self.beams.is_empty() == false {
            self.move_downward();
        }
    }

    fn move_downward(&mut self) {
        let mut next_beams = HashSet::new();

        for b in &self.beams {
            // Move the beam down
            let beam_down = Position { x: b.x, y: b.y + 1 };

            // Stop if the beam reached the end of the grid
            if beam_down.y > self.height { continue }

            // Otherwise, two possibilities:
            //  If the beam hits a splitter:
            //      "Activate" the splitter
            //      Push a splitted beam
            //  If the beam hits nothing,
            //      Simply push it to the active beams
            if let Some(splitter) = self.splitters.get_mut(&beam_down) {
                *splitter = true;
                vec![(-1, 0), (1, 0)].iter().for_each(|&(offset_x, offset_y)| {
                    if let Some(next_x) = beam_down.x.checked_add_signed(offset_x) {
                        if let Some(next_y) = beam_down.y.checked_add_signed(offset_y) {
                            if next_x < self.length && next_y < self.height {
                                next_beams.insert(Position { x: next_x, y: next_y });
                            }
                        }
                    }
                });
            } else {
                next_beams.insert(beam_down);
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

        assert_eq!(resolve(test_input), 21);
    }
}
