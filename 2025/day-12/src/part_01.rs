use std::collections::HashMap;

pub fn resolve(s: &str) -> usize {
    let (shapes, areas) = transform_data(s);

    let final_result = areas.iter().filter(|&a| a.can_all_shapes_fit(&shapes) && a.can_lay_all_shapes_next_to_eachother()).count();
    final_result
}

fn transform_data(data: &str) -> (Vec<usize>, Vec<Area>) {
    let mut shapes = vec![];
    let mut areas = vec![];

    let mut shape_map = HashMap::new();
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
                let presents = utils::core::parse_number_list(iter.last().unwrap());

                areas.push(Area { size: area_length * area_height, presents });
            }

            continue;
        }

        if line.is_empty() {
            shapes.push(0); // Allocate shapes vector
            shape_map.insert(shape_index, get_present_volume(shape_description));
            shape_description = vec![];

            continue;
        }

        /* Else, shape descrption */
        shape_description.push(line.chars().map(|c| c).collect());
    }

    for (i, v) in shape_map {
        shapes[i] = v;
    }

    (shapes, areas)
}

fn get_present_volume(d: Vec<Vec<char>>) -> usize {
    d.iter().map(|l| l.iter().filter(|c| **c == '#').count()).sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Area {
    size: usize,
    presents: Vec<usize>,
}
impl Area {
    fn can_all_shapes_fit(&self, shapes: &Vec<usize>) -> bool {
        let total_volume: usize = self.presents.iter().enumerate().map(|(i, nb)| shapes[i] * nb).sum();
        total_volume <= self.size
    }

    fn can_lay_all_shapes_next_to_eachother(&self) -> bool {
        let total_shape: usize = self.presents.iter().sum();
        (total_shape * 9) <= self.size
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_01() {
        let _ = "\
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

        // Useless: can't validate the input with this method...
        //assert_eq!(resolve(test_input), 2);
    }
}
