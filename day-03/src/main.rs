use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_file_in_memory(filepath: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        data.push(line.unwrap());
    }

    Ok(data)
}

fn transform_data(data: Vec<String>) -> Vec<Vec<char>> {
    let mut lines = Vec::new();

    for line in data {
        let mut cols = Vec::new();
        for char in line.chars() {
            cols.push(char);
        }
        lines.push(cols);
    }

    lines
}

fn grid_traversal(grid: &Vec<Vec<char>>) -> Vec<u32> {
    let symbols_positions = extract_symbols_position_from_grid(grid);
    let number_list_from_symbols: Vec<Vec<u32>> = symbols_positions.iter().map(|symbol_position| analyze_symbol(grid, symbol_position)).collect();

    let mut numbers = Vec::new();
    for mut number_list in number_list_from_symbols {
        numbers.append(&mut number_list);
    }
    numbers
}

struct Position {
    line: usize,
    column: usize,
}


fn analyze_symbol(grid: &Vec<Vec<char>>, index: &Position) -> Vec<u32> {
    let mut numbers_around_symbol = Vec::new();

    /* Searching for numbers on the line before, if it exists */
    if index.line > 0 {
        /* Search starting point is in the same as the symbol position, but one row above */
        let mut numbers_above = get_numbers_on_line(grid, &Position { line: index.line - 1, column: index.column });
        numbers_around_symbol.append(&mut numbers_above);
    }

    /* Searching for numbers before */
    if index.column > 0 {
        let mut left_numbers = get_numbers_ending_at_position(grid, &Position { line: index.line, column: index.column - 1});
        numbers_around_symbol.append(&mut left_numbers);
    }
    /* and after the symbol */
    if index.column + 1 < grid[index.line].len() {
        let mut right_numbers = get_numbers_starting_at_position(grid, &Position { line: index.line, column: index.column + 1});
        numbers_around_symbol.append(&mut right_numbers);
    }

    /* Searching for numbers on the line after, if it exists */
    if index.line + 1 < grid.len() {
        /* Search starting point is in the same as the symbol position, but one row below */
        let mut numbers_below = get_numbers_on_line(grid, &Position { line: index.line + 1, column: index.column });
        numbers_around_symbol.append(&mut numbers_below);
    }

    numbers_around_symbol
}

fn get_numbers_starting_at_position(grid: &Vec<Vec<char>>, position: &Position) -> Vec<u32> {
    if !(grid[position.line][position.column].is_numeric()) { return [].to_vec() }

    let end_index;
    if position.column + 1 >= grid[position.line].len() {
        end_index = position.column + 1;
    }
    else {
        end_index = find_end_index_of_number(grid, &Position { line: position.line, column: position.column + 1 });
    }

    [ get_number(&grid[position.line][position.column .. end_index]) ].to_vec()
}
fn find_end_index_of_number(grid: &Vec<Vec<char>>, position: &Position) -> usize {
    if !(grid[position.line][position.column].is_numeric()) { return position.column }
    if position.column + 1 >= grid[position.line].len() { return position.column + 1 }
    return find_end_index_of_number(grid, &Position { line: position.line, column: position.column + 1 })
}
fn get_numbers_ending_at_position(grid: &Vec<Vec<char>>, position: &Position) -> Vec<u32> {
    if !(grid[position.line][position.column].is_numeric()) { return [].to_vec() }

    /* column + 1 (since the end index is the first index after the number) */
    let end_index = position.column + 1;
    let start_index;
    if position.column == 0 {
        start_index = 0;
    } else {
        start_index = find_start_index_of_number(grid, &Position { line: position.line, column: position.column - 1 });
    }

    [ get_number(&grid[position.line][start_index .. end_index]) ].to_vec()
}
fn find_start_index_of_number(grid: &Vec<Vec<char>>, position: &Position) -> usize {
    if !(grid[position.line][position.column].is_numeric()) { return position.column + 1 }
    if position.column == 0 { return position.column }
    return find_start_index_of_number(grid, &Position { line: position.line, column: position.column - 1 })
}

fn get_numbers_on_line(grid: &Vec<Vec<char>>, index: &Position) -> Vec<u32> {
    let mut number_on_line = Vec::new();

    /* Check character at podition, 3 possibilites:
        . the character is '.' -> Check if there is a number ending on the left or starting on the right of this caracter
        . the character is a digit -> Extract this number and return it
        . the character is a symbol -> Do nothing, it will be analyzed on its own
     */
    match grid[index.line][index.column] {
        '.' => {
            /* Searching for numbers before */
            if index.column > 0 {
                number_on_line.append(&mut get_numbers_ending_at_position(grid, &Position { line: index.line, column: index.column - 1}));
            }
            /* and after the symbol */
            if index.column + 1 < grid[index.line].len() {
                number_on_line.append(&mut get_numbers_starting_at_position(grid, &Position { line: index.line, column: index.column + 1}));

            }
        },
        c => {
            if c.is_numeric() {
                let number_start_index = find_start_index_of_number(grid, index);
                let number_end_index = find_end_index_of_number(grid, index);

                number_on_line.push(get_number(&grid[index.line][number_start_index .. number_end_index]));
            } else {

            }
        }
    }

    number_on_line
}

fn get_number(grid: &[char]) -> u32 {
    let mut number = 0;

    for char in grid {
        number *= 10;
        number += char.to_digit(10).unwrap_or(0);
    }

    number
}

fn extract_symbols_position_from_grid(grid: &Vec<Vec<char>>) -> Vec<Position> {
    let mut symbols_positions = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                '.' => continue,
                c => {
                    if c.is_numeric() { continue; }
                    else { symbols_positions.push(Position { line: i, column: j }); }
                },
            }
        }
    }

    symbols_positions
}

fn test_01(grid: &Vec<Vec<char>>, line: usize, column: usize) {
    let test = analyze_symbol(&grid, &Position { line: line, column: column });
    println!("{}", grid[line][column]);
    for n in test {
        println!("[{}]", n);
    }
}

fn part_01() {
    let data = load_file_in_memory("./test-01.data").unwrap();
    let grid = transform_data(data);
    let number_list = grid_traversal(&grid);
    let final_result: u32 = number_list.iter().sum();

    test_01(&grid, 1, 3);
    test_01(&grid, 3, 6);
    test_01(&grid, 4, 3);
    test_01(&grid, 5, 5);
    test_01(&grid, 8, 3);
    test_01(&grid, 8, 5);

    println!("Extracted numbers: ");
    for i in number_list {
        println!("[{}]", i);
    }
    println!("- End -");


    println!("Part 1 final result: {}", final_result);
}

fn part_02() {
//    let data = load_file_in_memory("./test-02.data").unwrap();

}

fn main() {
    part_01();
    part_02();
}
