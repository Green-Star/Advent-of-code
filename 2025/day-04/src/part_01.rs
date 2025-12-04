pub fn resolve(s: &str) -> usize {
    let transformed_data = transform_data(s);
    let department = transformed_data.compute_neighbours();
    let final_result = department.rolls.iter().filter(|r| r.neighbours < 4).count();
    final_result
}

fn transform_data(data: &str) -> PrintingDepartment {
    let mut grid = vec![];
    let mut rolls = vec![];

    for (i, l) in data.lines().enumerate() {
        let mut v = vec![];
        for (j, c) in l.chars().enumerate() {
            match c {
                '@' => rolls.push(Roll { position: (i, j), neighbours: 0 }),
                _ => {},
            }
            v.push(c);
        }
        grid.push(v);
    }

    PrintingDepartment { grid, rolls }
}


#[derive(Debug)]
struct Roll {
    position: (usize, usize),
    neighbours: u8,
}

#[derive(Debug)]
struct PrintingDepartment {
    grid: Vec<Vec<char>>,
    rolls: Vec<Roll>,
}
impl PrintingDepartment {
    fn compute_neighbours(&self) -> PrintingDepartment{
        let computed_rolls: Vec<_> = self.rolls.iter().map(|r| Roll { position: r.position, neighbours: self.get_neighbours(r) }).collect();
        PrintingDepartment { grid: self.grid.clone(), rolls: computed_rolls }
    }

    fn get_neighbours(&self, roll: &Roll) -> u8 {
        let mut neighbours = 0;
        for offset_x in -1..=1 {
            for offset_y in -1..=1 {

//                println!("[{offset_x}, {offset_y}] ");

                println!("[{offset_x}, {offset_y}] -> {}", self.get_neighbours_at_position(&roll, (offset_x, offset_y)));
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
                println!("{:?}", self.rolls);
                if self.rolls.iter().find(|r| r.position.0 == x && r.position.1 == y).is_some() {
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
    fn test_part_01() {
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
