use std::collections::HashMap;

pub fn resolve(s: &str) -> usize {
    let (shapes, mut areas) = transform_data(s);
    //println!("{:?}", shapes);
//    println!("{:?}", areas);

    let final_result = areas.iter().filter(|&a| try_fit_all_shapes(&shapes, a)).count();
    final_result
}

fn transform_data(data: &str) -> (HashMap<usize, Vec<Shape>>, Vec<Area>) {
    let mut shapes = HashMap::new();
    let mut areas = vec![];

    let mut shape_index = 0;
    let mut shape_description: Vec<Vec<char>> = vec![];
    for line in data.lines() {
        // If the line contains a ":" -> Either a shape index or an area descrption
        if line.contains(":") {
            let mut iter = line.split(":");
            // Get only the interesting part
            let s = iter.next().unwrap();
            match s.contains("x") {
                true => {
                    /* Area description - see below */
                },
                false => {
                    /* Shape index */
                    shape_index = s.parse().unwrap();
                }
            }
            if s.contains("x") {
                let mut ss = s.split("x");
                let (area_length, area_height): (usize, usize) = (ss.next().unwrap().parse().unwrap(), ss.last().unwrap().parse().unwrap());
                let gifts = utils::core::parse_number_list(iter.last().unwrap());

                let space = (0..area_height).map(|_| AreaLine { used_space: 0, total_space: area_length }).collect();

                areas.push(Area { presents: gifts.clone(), presents_to_fit: gifts.clone(), space });
            }

            continue;
        }

        if line.is_empty() {
            shapes.insert(shape_index, Shape::rotate_from_shape_description(shape_description));
            shape_description = vec![];

            continue;
        }

        /* Else, shape descrption */
        shape_description.push(line.chars().map(|c| c).collect());
    }

    (shapes, areas)
}


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Shape {
    volume: Vec<usize>,
}
impl Shape {
    fn rotate_from_shape_description(d: Vec<Vec<char>>) -> Vec<Shape> {
        let mut base = vec![];
        for y in 0..d.len() {
            let mut occupied = 0;
            for x in 0..d[0].len() {
                if d[y][x] == '#' {
                    occupied += 1;
                }
            }
            base.push(occupied);
        }

        let mut rotate_90 = vec![];
        for x in 0..d[0].len() {
            let mut occupied = 0;
            for y in 0..d.len() {
                if d[y][x] == '#' {
                    occupied += 1;
                }
            }
            rotate_90.push(occupied);
        }

        vec![ Shape { volume: base.clone() }, Shape { volume: rotate_90.clone() }, Shape { volume: base.into_iter().rev().collect() }, Shape { volume: rotate_90.into_iter().rev().collect() } ]
    }
}

fn try_fit_shape(shape_index: &usize, shapes: &HashMap<usize, Vec<Shape>>, area: &Area) -> Option<Vec<Area>> {
    let shapes_to_place = shapes.get(shape_index).unwrap();

    let mut next_areas = vec![];

    for rotated_shape in shapes_to_place {
        if let Some(new_spaces) = try_fit_one_shape(&rotated_shape, area) {
            for u in new_spaces {
                next_areas.push(Area { presents: area.presents.clone(), presents_to_fit: area.presents_to_fit.clone(), space: u });
            }
        }
    }

    if next_areas.is_empty() { None } else { Some(next_areas) }
}

fn try_fit_all_shapes(shapes: &HashMap<usize, Vec<Shape>>, area: &Area) -> bool {
    let mut presents_to_fit = area.presents_to_fit.clone();
    let mut area_to_test = vec![ area.clone() ];

    loop {
        match presents_to_fit.iter().enumerate().filter(|(_, shape_number)| **shape_number > 0).last() {
            None => { println!("Done!"); return true },
            Some((shape_index, _)) => {
                println!("Placing shape {shape_index} in {} areas", area_to_test.len());
                presents_to_fit[shape_index] -= 1;

                let mut next_areas = vec![];
                for a in &area_to_test {
                    if let Some(mut areas) = try_fit_shape(&shape_index, shapes, a) {
                        next_areas.append(&mut areas);
                    }
                }

                area_to_test = next_areas;
                if area_to_test.is_empty() { println!("No soution!"); return false; }
            }
        }
    }


    true
}
// Try to fit one shape on all the lines of the given area
// Returns all updated areas (i.e. with the shape placed) if it succeeds, None if the shape can't fit at all
fn try_fit_one_shape(shape: &Shape, area: &Area) -> Option<Vec<Vec<AreaLine>>> {
    let mut result = vec![];

    for y in 0..area.space.len() {
        if try_fit_in_place(shape, y, area) {
            let mut updated_space = area.space.clone();
            for (i, length) in shape.volume.iter().enumerate() {
                updated_space[y + i].used_space += length;
            }
            result.push(updated_space);
        }
    }

    if result.is_empty() { None } else { Some(result) }
}
// Check if one particular shape can fit in one particular line of the given area
fn try_fit_in_place(shape: &Shape, place: usize, area: &Area) -> bool {
    for (y, shape_length) in shape.volume.iter().enumerate() {
        let line = place + y;
        if line >= area.space.len() {
            return false;
        }
        if area.space[line].used_space + shape_length > area.space[line].total_space {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AreaLine {
    used_space: usize,
    total_space: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Area {
    presents: Vec<usize>,
    presents_to_fit: Vec<usize>,

    space: Vec<AreaLine>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let test_input = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

        assert_eq!(resolve(test_input), 2);
    }


    #[test]
    fn test_first_test_only() {
        let test_input = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
";

        assert_eq!(resolve(test_input), 1);
    }

    #[test]
    fn test_create_shape_01() {
        let test_input = "\
0:
#.
#.
#.
##

";
        let (test, _) = transform_data(test_input);
        assert_eq!(*test.get(&0).unwrap(), vec![
                                            Shape { volume: vec![ 1, 1, 1, 2 ] },
                                            Shape { volume: vec![ 4, 1 ] },
                                            Shape { volume: vec![ 2, 1, 1, 1 ] },
                                            Shape { volume: vec![ 1, 4 ] },
                                        ]);
    }
    #[test]
    fn test_create_shape_02() {
        let test_input = "\
4:
###
#..
###

";

        let (test, _) = transform_data(test_input);
        assert_eq!(*test.get(&4).unwrap(), vec![
                                            Shape { volume: vec![ 3, 1, 3 ] },
                                            Shape { volume: vec![ 3, 2, 2 ] },
                                            Shape { volume: vec![ 3, 1, 3 ] },
                                            Shape { volume: vec![ 2, 2, 3 ] },
                                        ]);
    }
    #[test]
    fn test_create_shape_all() {
        let test_input = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

";

        let (test, _) = transform_data(test_input);
        assert_eq!(*test.get(&0).unwrap(), vec![
                                            Shape { volume: vec![ 3, 2, 2 ] },
                                            Shape { volume: vec![ 3, 3, 1 ] },
                                            Shape { volume: vec![ 2, 2, 3 ] },
                                            Shape { volume: vec![ 1, 3, 3 ] },
                                        ]);
        assert_eq!(*test.get(&1).unwrap(), vec![
                                            Shape { volume: vec![ 3, 2, 2 ] },
                                            Shape { volume: vec![ 2, 3, 2 ] },
                                            Shape { volume: vec![ 2, 2, 3 ] },
                                            Shape { volume: vec![ 2, 3, 2 ] },
                                        ]);
        assert_eq!(*test.get(&2).unwrap(), vec![
                                            Shape { volume: vec![ 2, 3, 2 ] },
                                            Shape { volume: vec![ 2, 3, 2 ] },
                                            Shape { volume: vec![ 2, 3, 2 ] },
                                            Shape { volume: vec![ 2, 3, 2 ] },
                                        ]);
        assert_eq!(*test.get(&3).unwrap(), vec![
                                            Shape { volume: vec![ 2, 3, 2 ] },
                                            Shape { volume: vec![ 3, 3, 1 ] },
                                            Shape { volume: vec![ 2, 3, 2 ] },
                                            Shape { volume: vec![ 1, 3, 3 ] },
                                        ]);
        assert_eq!(*test.get(&4).unwrap(), vec![
                                            Shape { volume: vec![ 3, 1, 3 ] },
                                            Shape { volume: vec![ 3, 2, 2 ] },
                                            Shape { volume: vec![ 3, 1, 3 ] },
                                            Shape { volume: vec![ 2, 2, 3 ] },
                                        ]);
        assert_eq!(*test.get(&5).unwrap(), vec![
                                            Shape { volume: vec![ 3, 1, 3 ] },
                                            Shape { volume: vec![ 2, 3, 2 ] },
                                            Shape { volume: vec![ 3, 1, 3 ] },
                                            Shape { volume: vec![ 2, 3, 2 ] },
                                        ]);
    }
}
