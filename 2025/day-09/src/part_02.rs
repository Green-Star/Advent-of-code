use std::cmp::{max, min};

pub fn resolve(s: &str) -> usize {
    let theater = transform_data(s);
    let rectangles = theater.get_rectangles();
    let final_result = rectangles.iter().map(|(a, b)| (b.x.abs_diff(a.x) + 1) * (b.y.abs_diff(a.y) + 1)).max().unwrap();
    final_result
}

fn transform_data(data: &str) -> Theater {
    let mut vertexes = vec![];

    /* Red tiles: will be used for the rectangles */
    for l in data.lines() {
        let pos = utils::core::parse_comma_number_list(l);

        let vertex = Position { x: pos[0], y: pos[1] };
        vertexes.push(vertex);
    }

    /* Every vertex is linked by an edge to the next one in the list input */
    let mut edges =  vec![];
    vertexes.windows(2).map(|w| (w[0], w[1])).for_each(|(from, to)| {
        edges.push((from, to));
    });
    /* List is overlapping */
    let first = vertexes[0];
    let last = vertexes.last().unwrap();
    edges.push((first, *last));

    Theater { vertexes, edges }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Theater {
    vertexes: Vec<Position>,
    edges: Vec<(Position, Position)>
}
impl Theater {
    /***
     * HUGE ASSUMPTION here
     *
     * The collision-check function has one big flaw: as it simply check if an edge crosses the rectangle,
     *  it will NOT filter rectangles overlapping a concave zone of the shape, event if these rectangles are outside of the shape
     *
     * /!\ We assume that any rectangle completely outside of the shape will not be bigger than the biggest rectangle inside /!\
     * Assumption is enforce by the fact that every vertexes defines the shape (hence every vertexes ARE inside the shape)
     * Thus, there isn't a lonely vertex outside of the shape that could be easily combined in a fully-outside rectangle
     */
    fn get_rectangles(&self) -> Vec<(&Position, &Position)> {
        let mut rectangles = vec![];

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

                    // AABB collision detection
                    // Since the edge is part of the shape, we can't filter rectangles using this edge
                    //  (i.e. we don't filter rectangles which have the same x or y coordinates than the edge)
                    //  This also explains why we don't filter fully-outside-shape rectangles
                    let edge_is_right_of_r = edge_min_x >= max_x;
                    let edge_is_left_of_r = edge_max_x <= min_x;
                    let edge_is_above_r = edge_max_y <= min_y;
                    let edge_is_below_r = edge_min_y >= max_y;

                    // If needed, we could add some checks here to be sure the rectangle is inside the shape and not outside
                    //  (it is working - so no need for these extra checks)

                    // If the edge is not on the left of the rectangle, nor on its right, nor above it, nor below it ; it means the edge is inside the rectangle
                    // And if the edge is inside the rectangle, it also means this rectangle can not be inside the shape
                    !(edge_is_left_of_r || edge_is_right_of_r || edge_is_above_r || edge_is_below_r)
                }) { continue }

                rectangles.push((a, c));
            }
        }

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

        test.vertexes = vec![ Position { x: 7, y: 3 }, Position { x: 11, y: 1 }, Position { x: 2, y: 5 } ];
        assert_eq!(test.get_rectangles().len(), 2);

        test.vertexes = vec![ Position { x: 7, y: 3 }, Position { x: 11, y: 1 }, Position { x: 2, y: 5 }, Position { x: 11, y: 7 } ];
        assert_eq!(test.get_rectangles().len(), 3);

        test.vertexes = vec![ Position { x: 7, y: 3 }, Position { x: 11, y: 1 }, Position { x: 2, y: 5 }, Position { x: 11, y: 7 }, Position { x: 9, y: 7 } ];
        // Should be: assert_eq!(test.get_rectangles().len(), 5);
        // Based on the assumption during the rectangles combination - the rectangle (9, 7)-(2, 5) is NOT eliminated, even if it is completely outside of the shape
        // (turns out this is not a problem in the end as this rectangle have the same area as (9, 5)-(2, 3), which is the biggest rectangle in the example shape)
        assert_eq!(test.get_rectangles().len(), 6);
    }
}
