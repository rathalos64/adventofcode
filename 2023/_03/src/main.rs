use std::env;
use std::path::Path;

mod part_01_02;
mod token;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    assert!(args.len() > 1); // first is always the filename
    let input_file: String = match args.pop() {
        None => panic!("No arguments given"),
        Some(input_file) => input_file
    };
    assert!(Path::new(&input_file).exists());

    println!("=== handle part 01 & 02 ===");
    let (part_numbers, sum_ratios): (i32, i32) = match part_01_02::run(&input_file) {
        Ok(result) => result,
        Err(e) => panic!("oh no {}", e)
    };
    println!("[part_numbers] = {}", part_numbers);
    println!("[sum_ratio] = {}", sum_ratios);
    println!();
}
