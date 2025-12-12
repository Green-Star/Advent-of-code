use std::collections::HashMap;

pub fn resolve(s: &str) -> i32 {
    let (shapes, areas) = transform_data(s);
    //println!("{:?}", shapes);
    println!("{:?}", areas);

    let final_result = 0;
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

                let space = (0..area_height).map(|_| (0, area_length)).collect();

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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Area {
    presents: Vec<usize>,
    presents_to_fit: Vec<usize>,

    space: Vec<(usize, usize)>,
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
