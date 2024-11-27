pub fn resolve(input_data_path: &str) {
    let _ = crate::core::load_file_in_memory(input_data_path).unwrap();

    let final_result = 0;

    println!("Part 1 final result: {}", final_result);
}
