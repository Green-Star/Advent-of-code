pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let grid = transform_data(data);
    let xmas = find_all_x_mas(&grid);

    let final_result = xmas.iter().filter(|o| **o).count();

    println!("Part 2 final result: {}", final_result);
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

fn find_all_x_mas(grid: &Vec<Vec<char>>) -> Vec<bool> {
    let mut result = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'A' {
                result.push(find_one_x_mas(grid, (i, j)));
            }
        }
    }

    result
}

fn find_one_x_mas(grid: &Vec<Vec<char>>, index: (usize, usize)) -> bool {
    let upper_left = (index.0.checked_add_signed(-1), index.1.checked_add_signed(-1));
    let upper_right = (index.0.checked_add_signed(1), index.1.checked_add_signed(-1));
    let bottom_left = (index.0.checked_add_signed(-1), index.1.checked_add_signed(1));
    let bottom_right = (index.0.checked_add_signed(1), index.1.checked_add_signed(1));

    let indexes = vec![upper_left, bottom_right, bottom_left, upper_right];
    if indexes.iter().all(|i| is_valid_index(grid, i)) == false { return false }

    let chars: Vec<char> = indexes.iter().map(|index| grid[index.0.unwrap()][index.1.unwrap()]).collect();
    let mas = vec![&(chars[..2]), &(chars[2..])];

    mas.iter().all(|v| is_mas(v))
}

fn is_valid_index(grid: &Vec<Vec<char>>, index: &(Option<usize>, Option<usize>)) -> bool {
    match index {
        (Some(i), Some(j)) => {
            if *i >= grid.len() { return false }
            if *j >= grid[*i].len() { return false }
        },
        _ => return false,
    }
    true
}

fn is_mas(vector: &[char]) -> bool {
    match (vector.iter().filter(|c| **c == 'M').count(), vector.iter().filter(|c| **c == 'S').count()) {
        (1, 1) => true,
        _ => false,
    }
}
