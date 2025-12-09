use std::{cmp::{max, min}, collections::{HashMap, HashSet}};

pub fn resolve(s: &str) -> usize {
    let transformed_data = transform_data(s);
//    let rectangles = create_all_rectangles(&transformed_data);
    let final_result = 0; //rectangles.iter().map(|(a, b)| (b.0.abs_diff(a.0) + 1) * (b.1.abs_diff(a.1) + 1)).max().unwrap();
    final_result
}

fn transform_data(data: &str) -> Theater {
    let mut vertexes = vec![];
    let mut min_x = None;
    let mut min_y = None;
    let mut max_x = None;
    let mut max_y = None;

    for l in data.lines() {
        let pos = utils::core::parse_comma_number_list(l);

        let vertex = Position { x: pos[0], y: pos[1] };
        vertexes.push(vertex);

        min_x = Some(min(vertex.x, min_x.unwrap_or(vertex.x)));
        min_y = Some(min(vertex.y, min_y.unwrap_or(vertex.y)));
        max_x = Some(max(vertex.x, max_x.unwrap_or(vertex.x)));
        max_y = Some(max(vertex.y, max_y.unwrap_or(vertex.y)));
    }

    /* Almost forget it:
    *
    * In your list, every red tile is connected to the red tile before and after it by a straight line of green tiles.
    * The list wraps, so the first red tile is also connected to the last red tile.
    * Tiles that are adjacent in your list will always be on either the same row or the same column.
    */
    let mut tiles = HashSet::new();

    vertexes.windows(2).map(|w| (w[0], w[1])).for_each(|(a, b)| {
        for x in min(a.x, b.x)..=max(a.x, b.x) {
            for y in min(a.y, b.y)..=max(a.y, b.y) {
                tiles.insert(Position { x, y });
            }
        }
    });
    /* List is overlapping */
    let first = vertexes[0];
    let last = vertexes.last().unwrap();
    for x in min(first.x, last.x)..=max(first.x, last.x) {
        for y in min(first.y, last.y)..=max(first.y, last.y) {
            tiles.insert(Position { x, y });
        }
    }

    /* And now fill in the tiles */
    for x in min_x.unwrap()..=max_x.unwrap() {
        let (min_y, max_y) = tiles.iter()
                                                                .filter(|&&p| p.x == x)
                                                                .map(|p| p.y)
                                                                .fold((None, None), |(min_y, max_y), y| (Some(min(y, min_y.unwrap_or(y))), Some(max(y, max_y.unwrap_or(y)))));

        for y in min_y.unwrap()..=max_y.unwrap() {
            tiles.insert(Position { x, y });
        }
    }

/****
 * In your list, every red tile is connected to the red tile before and after it by a straight line of green tiles.
 * The list wraps, so the first red tile is also connected to the last red tile.
 * Tiles that are adjacent in your list will always be on either the same row or the same column.
 *
 * => J'avais rate ca :)

    let mut green_column = HashMap::new();
    for x in min_x.unwrap()..=max_x.unwrap() {
        let mut green_line = false;
        for y in min_y.unwrap()..=min_y.unwrap() {
            match vertexes.get(&Position { x, y }) {
                Some(red_tile) => {
                    tiles.insert(*red_tile);
                    green_line = !green_line;
                    green_column.entry(y).and_modify(|is_green: &mut bool| *is_green = !*is_green).or_insert(true);
                },
                None => {
                    if green_line || *green_column.get(&y).unwrap_or(&false) {
                        tiles.insert(Position { x, y });
                    }
                },
            }


            tiles.insert(Position { x, y });
        }
    }

**/

    println!("{:?}", tiles);

    for y in 0..=8 {
        for x in 0..=13 {
            if let Some(_) = tiles.get(&Position { x, y }) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!("");
    }


    Theater { vertexes, tiles }
}

fn create_all_rectangles(vertexes: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    let mut rectangles = vec![];

    for (index, a) in vertexes.iter().enumerate() {
        for b in &vertexes[index+1..] {
            rectangles.push((*a, *b));
        }
    }

    rectangles
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Theater {
    vertexes: Vec<Position>,
    tiles: HashSet<Position>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_02() {
        let test_input = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

        assert_eq!(resolve(test_input), 24);
    }
}
