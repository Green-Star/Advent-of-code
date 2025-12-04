use std::collections::HashMap;

pub fn resolve(s: &str) -> usize {
    let transformed_data = transform_data(s);
    let department = transformed_data.compute_neighbours();
    let final_result = department.rolls.values().filter(|&&neighbours| neighbours < 4).count();
    final_result
}

fn transform_data(data: &str) -> PrintingDepartment {
    let mut rolls = HashMap::new();

    for (i, l) in data.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match c {
                '@' => { rolls.insert((i, j), 0); },
                _ => {},
            }
        }
    }

    PrintingDepartment { rolls }
}


#[derive(Debug)]
struct Roll {
    position: (usize, usize),
    neighbours: u8,
}

#[derive(Debug)]
struct PrintingDepartment {
    rolls: HashMap<(usize, usize), u8>,
}
impl PrintingDepartment {
    fn compute_neighbours(&self) -> PrintingDepartment{
        let computed_rolls: HashMap<(usize, usize), u8> = self.rolls.iter()
                                                                    .map(|(&position, _)| Roll { position, neighbours: 0 })
                                                                    .map(|r| (r.position, self.get_neighbours(&r)))
                                                                    .collect();
        PrintingDepartment { rolls: computed_rolls }
    }

    fn get_neighbours(&self, roll: &Roll) -> u8 {
        let mut neighbours = 0;
        for offset_x in -1..=1 {
            for offset_y in -1..=1 {
                neighbours += self.get_neighbours_at_position(&roll, (offset_x, offset_y));
            }
        }
        neighbours
    }
    fn get_neighbours_at_position(&self, roll: &Roll, offset: (isize, isize)) -> u8 {
        let (offset_x, offset_y) = offset;
        if offset_x == 0 && offset_y == 0 {
            return 0;
        }

        if let Some(x) = roll.position.0.checked_add_signed(offset_x) {
            if let Some(y) = roll.position.1.checked_add_signed(offset_y) {
                if let Some(_) = self.rolls.get(&(x, y)) {
                    return 1;
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn easy_setup_grid() -> PrintingDepartment {
        let test_input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        transform_data(test_input)
    }

    #[test]
    fn test_part_02() {
        let test_input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

        assert_eq!(resolve(test_input), 13);
    }

    #[test]
    fn test_neighbours_count_01() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours(&Roll { position: (0, 2), neighbours: 0}), 3);
    }
    #[test]
    fn test_neighbours_count_02() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours(&Roll { position: (0, 3), neighbours: 0}), 3);
    }
    #[test]
    fn test_neighbours_count_03() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours(&Roll { position: (0, 7), neighbours: 0}), 4);
    }
    #[test]
    fn test_neighbours_count_04() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours(&Roll { position: (0, 7), neighbours: 0}), 4);
    }
    #[test]
    fn test_neighbours_count_05() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours(&Roll { position: (4, 4), neighbours: 0}), 8);
    }
    #[test]
    fn test_neighbours_count_06() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours(&Roll { position: (2, 6), neighbours: 0}), 2);
    }
    #[test]
    fn test_neighbours_count_07() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours(&Roll { position: (5, 9), neighbours: 0}), 4);
    }
    #[test]
    fn test_neighbours_count_08() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours(&Roll { position: (7, 0), neighbours: 0}), 2);
    }
    #[test]
    fn test_neighbours_count_09() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours(&Roll { position: (9, 0), neighbours: 0}), 1);
    }
}
