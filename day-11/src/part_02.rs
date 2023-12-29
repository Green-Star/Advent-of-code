
const EXPAND_TIMES: usize = 100-1;

fn transform_data(data: Vec<String>) -> Vec<Galaxy> {
    let mut transformed = Vec::new();
    for i in 0..data.len() {
        transformed.push(Vec::new());
        for c in data[i].chars() {
            transformed[i].push(c);
        }
    }
    expand_space_and_get_galaxies(transformed)
}

fn expand_space_and_get_galaxies(data: Vec<Vec<char>>) -> Vec<Galaxy> {
    let expanded = expand_space(&data);
    extract_galaxies(data, expanded)
}

fn expand_space(data: &Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {
    let mut result = Vec::new();

    for _ in 0..data.len() {
        result.push(Vec::new());
    }

    /* Expand lines */
    let mut offset = 0;
    for i in 0..data.len() {
        let mut is_empty = true;
        for j in 0..data[i].len() {
            match data[i][j] {
                '.' => result[i].push((offset, 0)),
                _ => { result[i].push((offset, 0)); is_empty = false; },
            }
        }
        if is_empty { offset += 1 }
    }

    /* Expand columns */
    let mut offset = 0;
    for j in 0..data[0].len() {
        let mut is_empty = true;

        for i in 0..data.len() {
            match data[i][j] {
                '.' => result[i][j] = (result[i][j].0, offset),
                _ => { result[i][j] = (result[i][j].0, offset); is_empty = false; }
            }
        }

        if is_empty { offset += 1 }
    }

    result
}


fn extract_galaxies(map: Vec<Vec<char>>, expansion_offsets: Vec<Vec<(usize, usize)>>) -> Vec<Galaxy> {
    let mut result = Vec::new();

    for line in 0..map.len() {
        for column in 0..map[line].len() {
            match map[line][column] {
                '.' => {},
                _ => result.push(Galaxy { line: line + (expansion_offsets[line][column].0 * EXPAND_TIMES), column: column + (expansion_offsets[line][column].1 * EXPAND_TIMES) }),
            }
        }
    }

    result
}

fn get_pairs_of_galaxies(galaxies: Vec<Galaxy>) -> Vec<(Galaxy, Galaxy)> {
    let mut result = Vec::new();
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            result.push((galaxies[i], galaxies[j]))
        }
    }
    result
}

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    line: usize,
    column: usize,
}
impl Galaxy {
    fn get_distance(&self, other: &Galaxy) -> usize {
        self.line.abs_diff(other.line) + self.column.abs_diff(other.column)
    }
}

pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let galaxies = transform_data(data);
    let pairs = get_pairs_of_galaxies(galaxies);

    let final_result = pairs.iter().fold(0, |sum, (x, y)| sum + x.get_distance(y));

    println!("Part 2 final result: {}", final_result);
}
