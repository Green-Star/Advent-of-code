use std::{cmp::{max, min}, collections::HashSet, ops::{Index, IndexMut}};

pub fn resolve(s: &str) -> usize {
    let theater = transform_data(s);
    let rectangles = theater.get_rectangles();
    let final_result = rectangles.iter().map(|(a, b)| (b.x.abs_diff(a.x) + 1) * (b.y.abs_diff(a.y) + 1)).max().unwrap();
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

    /*
    let mut rectangles = vec![];
    for (index, a) in vertexes.iter().enumerate() {
        for b in &vertexes[index+1..] {
            rectangles.push((a, b));
        }
    }
    */
    let mut edges =  vec![];
    vertexes.windows(2).map(|w| (w[0], w[1])).for_each(|(from, to)| {
        edges.push((from, to));
    });
    /* List is overlapping */
    let first = vertexes[0];
    let last = vertexes.last().unwrap();
    edges.push((first, *last));

//    println!("{} rectangles", rectangles.len());
    println!("with {} edges", edges.len());

    return Theater { vertexes, tiles: HashSet::new(), edges };

    println!("Kill it!");

    /* Almost forget it:
    *
    * In your list, every red tile is connected to the red tile before and after it by a straight line of green tiles.
    * The list wraps, so the first red tile is also connected to the last red tile.
    * Tiles that are adjacent in your list will always be on either the same row or the same column.
    */
    let mut border_tiles = HashSet::new();

    vertexes.windows(2).map(|w| (w[0], w[1])).for_each(|(a, b)| {
        for x in min(a.x, b.x)..=max(a.x, b.x) {
            for y in min(a.y, b.y)..=max(a.y, b.y) {
                border_tiles.insert(Position { x, y });
            }
        }
    });
    /* List is overlapping */
    let first = vertexes[0];
    let last = vertexes.last().unwrap();
    for x in min(first.x, last.x)..=max(first.x, last.x) {
        for y in min(first.y, last.y)..=max(first.y, last.y) {
            border_tiles.insert(Position { x, y });
        }
    }
    println!("Starting tiles ({},{}) -> {},{}", min_x.unwrap(), min_y.unwrap(), max_x.unwrap(), max_y.unwrap());

    /* And now fill in the tiles */
    let mut tiles = HashSet::new();
    for x in min_x.unwrap()..=max_x.unwrap() {
        let (min_y, max_y) = border_tiles.iter()
                                .filter(|&&p| p.x == x)
                                .map(|p| p.y)
                                .fold((None, None), |(min_y, max_y), y| (Some(min(y, min_y.unwrap_or(y))), Some(max(y, max_y.unwrap_or(y)))));

        if x % 10000 == 0 {
            println!("At col x={}, {} tiles to process", x, border_tiles.len());
        }

        for y in min_y.unwrap()..=max_y.unwrap() {
            tiles.insert(Position { x, y });
        }
    }

    println!("Tiles: {:?}", tiles);

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

    Theater { vertexes, tiles, edges: vec![] }
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
    edges: Vec<(Position, Position)>
}
impl Theater {
    fn get_rectangles(&self) -> Vec<(&Position, &Position)> {
        let mut rectangles = vec![];

        /*
        for (index, a) in self.vertexes.iter().enumerate() {
            for c in &self.vertexes[index+1..] {
                let b = Position { x: a.x, y: c.y };
                let d = Position { x: c.x, y: a.y };

                let is_y_valid = (min(a.y, c.y) ..= max(a.y, c.y))
                    .flat_map(|y| vec![ Position { x: a.x, y }, Position { x: c.x, y } ])
                    .all(|p| self.tiles.get(&p).is_some());
                if is_y_valid == false { continue }

                let is_x_valid = (min(a.x, c.x)..=(max(a.x, c.x)))
                    .flat_map(|x| vec![ Position { x, y: a.y }, Position { x, y: c.y }])
                    .all(|p| self.tiles.get(&p).is_some());
                if is_x_valid == false { continue }

                rectangles.push((a, c));
            }

        }
        */

        for (index, a) in self.vertexes.iter().enumerate() {
            for c in &self.vertexes[index+1..] {
                let min_x = min(a.x, c.x);
                let max_x = max(a.x, c.x);
                let min_y = min(a.y, c.y);
                let max_y = max(a.y, c.y);

                if self.edges.iter().any(|(x, y)| {
                    let edge_min_x = min(x.x, y.x);
                    let edge_max_x = max(x.x, y.x);
                    let edge_min_y = min(x.y, y.y);
                    let edge_max_y = max(x.y, y.y);
/*
                    let horizontal_collide = false;

                    let vertical_collide = min_y > edge_min_y && max_y < edge_max_y;

                    let vertical_collide = true;

                    horizontal_collide || vertical_collide
                    */

                    let edge_is_right_of_r = edge_min_x >= max_x;
                    let edge_is_left_of_r = edge_max_x <= min_x;
                    let edge_is_above_r = edge_max_y <= min_y;
                    let edge_is_below_r = edge_min_y >= max_y;

                    let collide = !(edge_is_right_of_r || edge_is_left_of_r || edge_is_above_r || edge_is_below_r);
                    if collide { println!("({a:?}-{c:?} collide with {x:?}-{y:?}") }

                    collide
                }) { continue }

                rectangles.push((a, c));
            }
        }

        println!("Here's the rectangles");
        println!("{:?}", rectangles);

        rectangles
    }
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


    #[test]
    fn test_combine_few_rectangles() {
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
        let mut test = transform_data(test_input);
        assert_eq!(test.vertexes.len(), 8);
//        assert_eq!(test.tiles.len(), 46);

        test.vertexes = vec![ Position { x: 7, y: 3 }, Position { x: 11, y: 1 }, Position { x: 3, y: 5 } ];
        assert_eq!(test.get_rectangles().len(), 2);

        test.vertexes = vec![ Position { x: 7, y: 3 }, Position { x: 11, y: 1 }, Position { x: 3, y: 5 }, Position { x: 11, y: 7 } ];
        assert_eq!(test.get_rectangles().len(), 3);

        test.vertexes = vec![ Position { x: 7, y: 3 }, Position { x: 11, y: 1 }, Position { x: 3, y: 5 }, Position { x: 11, y: 7 }, Position { x: 9, y: 7 } ];
        assert_eq!(test.get_rectangles().len(), 5);
    }
}
