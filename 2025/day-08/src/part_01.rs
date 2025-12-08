use std::cmp::Ordering;

pub fn resolve(s: &str) -> usize {
    resolve_with_step(s, 1000)
}

fn resolve_with_step(s: &str, step: u32) -> usize {
    let mut playground = transform_data(s);

    for _ in 1..=step {
        playground.compute_step();
    }

    let final_result = playground.get_three_largest_size();
    final_result
}

fn transform_data(data: &str) -> Playground {
    let mut boxes = vec![];
    let mut circuits= vec![];

    for l in data.lines() {
        let v = utils::core::parse_comma_number_list(l);
        let (x, y, z) = (v[0], v[1], v[2]);
        let b = Box { x, y, z };
        boxes.push(b);
        // At first, each box is in its own circuit
        circuits.push(vec![ b ]);
    }

    let mut distances = Playground::compute_all_distances(&boxes);
    // DESCENDING order on the distances (so we will be able to get the closest boxes later on, simply by pop()ing the vector)
    distances.sort_by(|(_, _, x), (_, _, y)| if x > y { Ordering::Less } else if x < y { Ordering::Greater } else { Ordering::Equal });

    Playground { boxes, distances, circuits }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Box {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Playground {
    boxes: Vec<Box>,
    distances: Vec<(Box, Box, f64)>,
    circuits: Vec<Vec<Box>>,
}
impl Playground {
    fn compute_all_distances(boxes: &Vec<Box>) -> Vec<(Box, Box, f64)> {
        let mut distances = vec![];

        for (i, a) in boxes.iter().enumerate() {
            for b in &boxes[i+1..] {
                distances.push((*a, *b, Self::compute_distance_between_boxes(a, b)));
            }
        }

        distances
    }
    fn compute_distance_between_boxes(a: &Box, b: &Box) -> f64 {
        f64::sqrt(
                ((b.x - a.x) as f64 * (b.x - a.x) as f64) +
                ((b.y - a.y) as f64 * (b.y - a.y) as f64) +
                ((b.z - a.z) as f64 * (b.z - a.z) as f64)
        )
    }

    fn compute_step(&mut self) {
        let (a, b, _) = self.distances.pop().unwrap();
        self.merge_ciruit(&a, &b);
    }

    fn merge_ciruit(&mut self, a: &Box, b: &Box) {
        // Find box A circuit and remove it from the circuit vector
        let mut circuit_a = self.circuits.remove(self.circuits.iter().enumerate().find(|(_, c)| c.contains(a)).unwrap().0);

        // Find box B circuit
        //  2 possibilities:
        //  If the box B circuit is still in the circuit vector,
        //      Remove it, merge the two circuit, and push the merged circuit back into the circuit vector
        //  If the box B circuit is not find in the circuit vector, it means box A & B are already connected to the same cirecuit
        //      In that case, simply push back the circuit in the circuit vector
        let other = self.circuits.iter().enumerate().find(|(_, c)| c.contains(b));
        if let Some((index, _)) = other {
            let mut circuit_b = self.circuits.remove(index);
            circuit_a.append(&mut circuit_b);
            self.circuits.push(circuit_a);
        } else {
            self.circuits.push(circuit_a);
        }
    }

    fn get_three_largest_size(&self) -> usize {
        let mut circuits = self.circuits.clone();
        circuits.sort_by(|a, b| a.len().cmp(&b.len()));
        let a = circuits.pop().unwrap();
        let b = circuits.pop().unwrap();
        let c = circuits.pop().unwrap();
        a.len() * b.len() * c.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn easy_setup_test_data() -> Playground {
        Playground {
            boxes: vec![
                Box { x: 162, y: 817, z: 812 }, Box { x: 57, y: 618, z: 57 }, Box { x: 906, y: 360, z: 560 },
                Box { x: 431, y: 825, z: 988 }, Box { x: 805, y: 96, z: 715 }, Box { x: 970, y: 615, z: 88 },
                Box { x: 425, y: 690, z: 689 },
            ],
            distances: vec![
                (Box { x: 352, y: 342, z: 300 }, Box { x: 466, y: 668, z: 158 }, 373.41130138226936),
                (Box { x: 352, y: 342, z: 300 }, Box { x: 117, y: 168, z: 530 }, 372.02284876066415),
                (Box { x: 352, y: 342, z: 300 }, Box { x: 542, y: 29, z: 236 }, 371.70552861102294),
                (Box { x: 592, y: 479, z: 940 }, Box { x: 425, y: 690, z: 689 }, 367.9823365326113),
                (Box { x: 906, y: 360, z: 560 }, Box { x: 984, y: 92, z: 344 }, 352.936254867646),
                (Box { x: 346, y: 949, z: 466 }, Box { x: 425, y: 690, z: 689 }, 350.786259708102),
                (Box { x: 906, y: 360, z: 560 }, Box { x: 739, y: 650, z: 466 }, 347.59890678769403),
                (Box { x: 819, y: 987, z: 18 }, Box { x: 941, y: 993, z: 340 }, 344.3893145845266),
                (Box { x: 52, y: 470, z: 668 }, Box { x: 117, y: 168, z: 530 }, 338.33858780813046),
                (Box { x: 862, y: 61, z: 35 }, Box { x: 984, y: 92, z: 344 }, 333.6555109690233),
                (Box { x: 431, y: 825, z: 988 }, Box { x: 425, y: 690, z: 689 }, 328.11888089532425),
                (Box { x: 906, y: 360, z: 560 }, Box { x: 805, y: 96, z: 715 }, 322.36935338211043),
                (Box { x: 162, y: 817, z: 812 }, Box { x: 431, y: 825, z: 988 }, 321.560258738545),
                (Box { x: 162, y: 817, z: 812 }, Box { x: 425, y: 690, z: 689 }, 316.90219311326956)
            ],
            circuits: vec![
                vec![Box { x: 162, y: 817, z: 812 }], vec![Box { x: 57, y: 618, z: 57 }], vec![Box { x: 906, y: 360, z: 560 }],
                vec![Box { x: 431, y: 825, z: 988 }], vec![Box { x: 805, y: 96, z: 715 }], vec![Box { x: 970, y: 615, z: 88 }],
                vec![Box { x: 425, y: 690, z: 689 }],
            ],
        }
    }


    #[test]
    fn test_part_01() {
        let test_input = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

        assert_eq!(resolve_with_step(test_input, 10), 40);
    }


    #[test]
    fn test_step_01() {
        let expected = Playground {
            boxes: vec![
                Box { x: 162, y: 817, z: 812 }, Box { x: 57, y: 618, z: 57 }, Box { x: 906, y: 360, z: 560 },
                Box { x: 431, y: 825, z: 988 }, Box { x: 805, y: 96, z: 715 }, Box { x: 970, y: 615, z: 88 },
                Box { x: 425, y: 690, z: 689 },
            ],
            distances: vec![
                (Box { x: 352, y: 342, z: 300 }, Box { x: 466, y: 668, z: 158 }, 373.41130138226936),
                (Box { x: 352, y: 342, z: 300 }, Box { x: 117, y: 168, z: 530 }, 372.02284876066415),
                (Box { x: 352, y: 342, z: 300 }, Box { x: 542, y: 29, z: 236 }, 371.70552861102294),
                (Box { x: 592, y: 479, z: 940 }, Box { x: 425, y: 690, z: 689 }, 367.9823365326113),
                (Box { x: 906, y: 360, z: 560 }, Box { x: 984, y: 92, z: 344 }, 352.936254867646),
                (Box { x: 346, y: 949, z: 466 }, Box { x: 425, y: 690, z: 689 }, 350.786259708102),
                (Box { x: 906, y: 360, z: 560 }, Box { x: 739, y: 650, z: 466 }, 347.59890678769403),
                (Box { x: 819, y: 987, z: 18 }, Box { x: 941, y: 993, z: 340 }, 344.3893145845266),
                (Box { x: 52, y: 470, z: 668 }, Box { x: 117, y: 168, z: 530 }, 338.33858780813046),
                (Box { x: 862, y: 61, z: 35 }, Box { x: 984, y: 92, z: 344 }, 333.6555109690233),
                (Box { x: 431, y: 825, z: 988 }, Box { x: 425, y: 690, z: 689 }, 328.11888089532425),
                (Box { x: 906, y: 360, z: 560 }, Box { x: 805, y: 96, z: 715 }, 322.36935338211043),
                (Box { x: 162, y: 817, z: 812 }, Box { x: 431, y: 825, z: 988 }, 321.560258738545),
            ],
            circuits: vec![
                vec![Box { x: 57, y: 618, z: 57 }], vec![Box { x: 906, y: 360, z: 560 }],
                vec![Box { x: 431, y: 825, z: 988 }], vec![Box { x: 805, y: 96, z: 715 }], vec![Box { x: 970, y: 615, z: 88 },],
                vec![Box { x: 162, y: 817, z: 812 }, Box { x: 425, y: 690, z: 689 }],
            ],
        };

        let mut test_data = easy_setup_test_data();
        test_data.compute_step();
        assert_eq!(test_data, expected);
    }
    #[test]
    fn test_step_02() {
        let expected = Playground {
            boxes: vec![
                Box { x: 162, y: 817, z: 812 }, Box { x: 57, y: 618, z: 57 }, Box { x: 906, y: 360, z: 560 },
                Box { x: 431, y: 825, z: 988 }, Box { x: 805, y: 96, z: 715 }, Box { x: 970, y: 615, z: 88 },
                Box { x: 425, y: 690, z: 689 },
            ],
            distances: vec![
                (Box { x: 352, y: 342, z: 300 }, Box { x: 466, y: 668, z: 158 }, 373.41130138226936),
                (Box { x: 352, y: 342, z: 300 }, Box { x: 117, y: 168, z: 530 }, 372.02284876066415),
                (Box { x: 352, y: 342, z: 300 }, Box { x: 542, y: 29, z: 236 }, 371.70552861102294),
                (Box { x: 592, y: 479, z: 940 }, Box { x: 425, y: 690, z: 689 }, 367.9823365326113),
                (Box { x: 906, y: 360, z: 560 }, Box { x: 984, y: 92, z: 344 }, 352.936254867646),
                (Box { x: 346, y: 949, z: 466 }, Box { x: 425, y: 690, z: 689 }, 350.786259708102),
                (Box { x: 906, y: 360, z: 560 }, Box { x: 739, y: 650, z: 466 }, 347.59890678769403),
                (Box { x: 819, y: 987, z: 18 }, Box { x: 941, y: 993, z: 340 }, 344.3893145845266),
                (Box { x: 52, y: 470, z: 668 }, Box { x: 117, y: 168, z: 530 }, 338.33858780813046),
                (Box { x: 862, y: 61, z: 35 }, Box { x: 984, y: 92, z: 344 }, 333.6555109690233),
                (Box { x: 431, y: 825, z: 988 }, Box { x: 425, y: 690, z: 689 }, 328.11888089532425),
                (Box { x: 906, y: 360, z: 560 }, Box { x: 805, y: 96, z: 715 }, 322.36935338211043),
            ],
            circuits: vec![
                vec![Box { x: 57, y: 618, z: 57 }], vec![Box { x: 906, y: 360, z: 560 }],
                vec![Box { x: 805, y: 96, z: 715 }], vec![Box { x: 970, y: 615, z: 88 }],
                vec![Box { x: 162, y: 817, z: 812 }, Box { x: 425, y: 690, z: 689 }, Box { x: 431, y: 825, z: 988 }],
            ],
        };

        let mut test_data = easy_setup_test_data();
        test_data.compute_step();
        test_data.compute_step();
        assert_eq!(test_data, expected);
    }
    #[test]
    fn test_step_03() {
        let expected = Playground {
            boxes: vec![
                Box { x: 162, y: 817, z: 812 }, Box { x: 57, y: 618, z: 57 }, Box { x: 906, y: 360, z: 560 },
                Box { x: 431, y: 825, z: 988 }, Box { x: 805, y: 96, z: 715 }, Box { x: 970, y: 615, z: 88 },
                Box { x: 425, y: 690, z: 689 },
            ],
            distances: vec![
                (Box { x: 352, y: 342, z: 300 }, Box { x: 466, y: 668, z: 158 }, 373.41130138226936),
                (Box { x: 352, y: 342, z: 300 }, Box { x: 117, y: 168, z: 530 }, 372.02284876066415),
                (Box { x: 352, y: 342, z: 300 }, Box { x: 542, y: 29, z: 236 }, 371.70552861102294),
                (Box { x: 592, y: 479, z: 940 }, Box { x: 425, y: 690, z: 689 }, 367.9823365326113),
                (Box { x: 906, y: 360, z: 560 }, Box { x: 984, y: 92, z: 344 }, 352.936254867646),
                (Box { x: 346, y: 949, z: 466 }, Box { x: 425, y: 690, z: 689 }, 350.786259708102),
                (Box { x: 906, y: 360, z: 560 }, Box { x: 739, y: 650, z: 466 }, 347.59890678769403),
                (Box { x: 819, y: 987, z: 18 }, Box { x: 941, y: 993, z: 340 }, 344.3893145845266),
                (Box { x: 52, y: 470, z: 668 }, Box { x: 117, y: 168, z: 530 }, 338.33858780813046),
                (Box { x: 862, y: 61, z: 35 }, Box { x: 984, y: 92, z: 344 }, 333.6555109690233),
                (Box { x: 431, y: 825, z: 988 }, Box { x: 425, y: 690, z: 689 }, 328.11888089532425),
            ],
            circuits: vec![
                vec![Box { x: 57, y: 618, z: 57 }], vec![Box { x: 970, y: 615, z: 88 }],
                vec![Box { x: 162, y: 817, z: 812 }, Box { x: 425, y: 690, z: 689 }, Box { x: 431, y: 825, z: 988 }],
                vec![Box { x: 906, y: 360, z: 560 }, Box { x: 805, y: 96, z: 715 }],
            ],
        };

        let mut test_data = easy_setup_test_data();
        test_data.compute_step();
        test_data.compute_step();
        test_data.compute_step();
        assert_eq!(test_data, expected);
    }
    #[test]
    fn test_step_04() {
        let expected = Playground {
            boxes: vec![
                Box { x: 162, y: 817, z: 812 }, Box { x: 57, y: 618, z: 57 }, Box { x: 906, y: 360, z: 560 },
                Box { x: 431, y: 825, z: 988 }, Box { x: 805, y: 96, z: 715 }, Box { x: 970, y: 615, z: 88 },
                Box { x: 425, y: 690, z: 689 },
            ],
            distances: vec![
                (Box { x: 352, y: 342, z: 300 }, Box { x: 466, y: 668, z: 158 }, 373.41130138226936),
                (Box { x: 352, y: 342, z: 300 }, Box { x: 117, y: 168, z: 530 }, 372.02284876066415),
                (Box { x: 352, y: 342, z: 300 }, Box { x: 542, y: 29, z: 236 }, 371.70552861102294),
                (Box { x: 592, y: 479, z: 940 }, Box { x: 425, y: 690, z: 689 }, 367.9823365326113),
                (Box { x: 906, y: 360, z: 560 }, Box { x: 984, y: 92, z: 344 }, 352.936254867646),
                (Box { x: 346, y: 949, z: 466 }, Box { x: 425, y: 690, z: 689 }, 350.786259708102),
                (Box { x: 906, y: 360, z: 560 }, Box { x: 739, y: 650, z: 466 }, 347.59890678769403),
                (Box { x: 819, y: 987, z: 18 }, Box { x: 941, y: 993, z: 340 }, 344.3893145845266),
                (Box { x: 52, y: 470, z: 668 }, Box { x: 117, y: 168, z: 530 }, 338.33858780813046),
                (Box { x: 862, y: 61, z: 35 }, Box { x: 984, y: 92, z: 344 }, 333.6555109690233),
            ],
            circuits: vec![
                vec![Box { x: 57, y: 618, z: 57 }], vec![Box { x: 970, y: 615, z: 88 }],
                vec![Box { x: 906, y: 360, z: 560 }, Box { x: 805, y: 96, z: 715 }],
                vec![Box { x: 162, y: 817, z: 812 }, Box { x: 425, y: 690, z: 689 }, Box { x: 431, y: 825, z: 988 }],
            ],
        };

        let mut test_data = easy_setup_test_data();
        test_data.compute_step();
        test_data.compute_step();
        test_data.compute_step();
        test_data.compute_step();
        assert_eq!(test_data, expected);
    }


    #[test]
    fn test_step_10() {
        let test_input = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

        let mut test = transform_data(test_input);
        for _ in 1..=10 {
            test.compute_step();
        }
        // Too lazy to get all the details, simply use the ont mentioned in the problem
        assert_eq!(test.circuits.len(), 11);
        let mut circuits = test.circuits;
        // Sort circuit by length in ascending order
        circuits.sort_by(|a, b| a.len().cmp(&b.len()));
        assert_eq!(circuits.pop().unwrap().len(), 5);
        assert_eq!(circuits.pop().unwrap().len(), 4);
        assert_eq!(circuits.pop().unwrap().len(), 2);
        assert_eq!(circuits.pop().unwrap().len(), 2);
        // Only units circuits should remain
        assert_eq!(circuits.len(), 7);
    }
}
