use std::str::Chars;

pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let grid = transform_data(data);
    let xmas = find_all_xmas(&grid);

    println!("{:?} match", xmas.len());

    let final_result = xmas.iter().fold(0, |sum, xmas| sum + xmas);

    println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    for l in data {
        let mut line = Vec::new();
        for c in l.chars() {
            line.push(c);
        }
        result.push(line);
    }

    result
}

fn find_all_xmas(grid: &Vec<Vec<char>>) -> Vec<i32> {
    let mut needle = "XMAS".chars();

    let mut result = Vec::new();

    let c = needle.next().unwrap();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == c {
                result.push(explore_from_xmas(grid, (i, j), needle.clone()));
            }
        }
    }

    result
}

fn explore_from_xmas<'a>(grid: &Vec<Vec<char>>, index: (usize, usize), needle: Chars<'a>) -> i32
{
    let result = vec![
        explore_in_one_direction(grid, index, needle.clone(), (-1, -1)),
        explore_in_one_direction(grid, index, needle.clone(), (-1, 0)),
        explore_in_one_direction(grid, index, needle.clone(), (-1, 1)),
        explore_in_one_direction(grid, index, needle.clone(), (0, -1)),
        explore_in_one_direction(grid, index, needle.clone(), (0, 1)),
        explore_in_one_direction(grid, index, needle.clone(), (1, -1)),
        explore_in_one_direction(grid, index, needle.clone(), (1, 0)),
        explore_in_one_direction(grid, index, needle.clone(), (1, 1)),
    ];

    println!("Result: {:?}", result);

    result.iter().fold(0, |acc, o| {
        match o {
            Some(value) => acc + value,
            None => acc
        }
    })
}

fn explore_in_one_direction<'a>(grid: &Vec<Vec<char>>, index: (usize, usize), mut needle: Chars<'a>, offset: (isize, isize)) -> Option<i32>
{
    let current_char;
    match needle.next() {
        None => return Some(1),
        Some(char) => current_char = char,
    }

    let current_index = (index.0.checked_add_signed(offset.0), index.1.checked_add_signed(offset.1));

    let x;
    let y;
    match current_index {
        (Some(i), Some(j)) => { x = i; y = j; },
        _ => { return None },
    }
    if x >= grid.len() { return None }
    if y >= grid[x].len() { return None }

    if grid[x][y] != current_char { return None }

    explore_in_one_direction(grid, (x, y), needle.clone(), offset)
}
