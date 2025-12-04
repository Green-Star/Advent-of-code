use std::collections::HashMap;

pub fn resolve(s: &str) -> usize {
    let transformed_data = transform_data(s);

    let mut final_result = 0;

    let (mut removed, mut grid) = transformed_data.remove();
    while removed > 0 {
        final_result += removed;
        println!("{removed} removed at this iteration ({final_result} total)");
        (removed, grid) = grid.remove();
    }

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
struct PrintingDepartment {
    rolls: HashMap<(usize, usize), u8>,
}
impl PrintingDepartment {
    fn compute_neighbours(&self) -> PrintingDepartment{
        let computed_rolls: HashMap<(usize, usize), u8> = self.rolls.iter()
                                                                    .map(|(position, _)| (*position, self.get_neighbours_for_roll(position)))
                                                                    .collect();
        PrintingDepartment { rolls: computed_rolls }
    }

    fn get_neighbours_for_roll(&self, roll: &(usize, usize)) -> u8 {
        let mut neighbours = 0;
        for offset_x in -1..=1 {
            for offset_y in -1..=1 {
                neighbours += self.get_neighbour_at_position(&roll, (offset_x, offset_y));
            }
        }
        neighbours
    }
    fn get_neighbour_at_position(&self, position: &(usize, usize), offset: (isize, isize)) -> u8 {
        let (offset_x, offset_y) = offset;
        if offset_x == 0 && offset_y == 0 {
            return 0;
        }

        if let Some(x) = position.0.checked_add_signed(offset_x) {
            if let Some(y) = position.1.checked_add_signed(offset_y) {
                if let Some(_) = self.rolls.get(&(x, y)) {
                    return 1;
                }
            }
        }

        0
    }

    fn remove(&self) -> (usize, PrintingDepartment) {
        let remaining_rolls: HashMap<(usize, usize), u8> = self.compute_neighbours().rolls.iter()
                                                                                        .filter(|(_, &neighbour)| neighbour >= 4)
                                                                                        .map(|(position, n)| (*position, *n))
                                                                                        .collect();
        (self.rolls.len() - remaining_rolls.len(), PrintingDepartment { rolls: remaining_rolls })
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

        assert_eq!(resolve(test_input), 43);
    }

    #[test]
    fn test_neighbours_count_01() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours_for_roll(&(0, 2)), 3);
    }
    #[test]
    fn test_neighbours_count_02() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours_for_roll(&(0, 3)), 3);
    }
    #[test]
    fn test_neighbours_count_03() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours_for_roll(&(0, 7)), 4);
    }
    #[test]
    fn test_neighbours_count_04() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours_for_roll(&(0, 7)), 4);
    }
    #[test]
    fn test_neighbours_count_05() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours_for_roll(&(4, 4)), 8);
    }
    #[test]
    fn test_neighbours_count_06() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours_for_roll(&(2, 6)), 2);
    }
    #[test]
    fn test_neighbours_count_07() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours_for_roll(&(5, 9)), 4);
    }
    #[test]
    fn test_neighbours_count_08() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours_for_roll(&(7, 0)), 2);
    }
    #[test]
    fn test_neighbours_count_09() {
        let grid = easy_setup_grid();
        assert_eq!(grid.get_neighbours_for_roll(&(9, 0)), 1);
    }


    #[test]
    fn test_removing_step_01() {
        let grid = easy_setup_grid();
        let (removed, _) = grid.remove();
        assert_eq!(removed, 13);
    }
    #[test]
    fn test_removing_step_02() {
        let grid = easy_setup_grid();
        let (_, grid) = grid.remove();
        let (removed, _) = grid.remove();
        assert_eq!(removed, 12);
    }    #[test]
    fn test_removing_step_03() {
        let grid = easy_setup_grid();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (removed, _) = grid.remove();
        assert_eq!(removed, 7);
    }    #[test]
    fn test_removing_step_04() {
        let grid = easy_setup_grid();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (removed, _) = grid.remove();
        assert_eq!(removed, 5);
    }    #[test]
    fn test_removing_step_05() {
        let grid = easy_setup_grid();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (removed, _) = grid.remove();
        assert_eq!(removed, 2);
    }
    #[test]
    fn test_removing_step_06() {
        let grid = easy_setup_grid();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (removed, _) = grid.remove();
        assert_eq!(removed, 1);
    }    #[test]
    fn test_removing_step_07() {
        let grid = easy_setup_grid();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (removed, _) = grid.remove();
        assert_eq!(removed, 1);
    }    #[test]
    fn test_removing_step_08() {
        let grid = easy_setup_grid();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (removed, _) = grid.remove();
        assert_eq!(removed, 1);
    }    #[test]
    fn test_removing_step_09() {
        let grid = easy_setup_grid();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (removed, _) = grid.remove();
        assert_eq!(removed, 1);
    }    #[test]
    fn test_removing_step_10() {
        let grid = easy_setup_grid();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (_, grid) = grid.remove();
        let (removed, _) = grid.remove();
        assert_eq!(removed, 0);
    }
}
