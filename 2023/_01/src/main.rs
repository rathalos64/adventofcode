use std::env;
use std::path::Path;

mod part_one;
mod part_two;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    assert!(args.len() > 1); // first is always the filename
    let input_file: String = match args.pop() {
        None => panic!("No arguments given"),
        Some(input_file) => input_file
    };
    assert!(Path::new(&input_file).exists());

    println!("=== handle part one ===");
    let calibration_values: u32 = match part_one::run(&input_file) {
        Ok(calibration_values) => calibration_values,
        Err(e) => panic!("{}", e)
    };
    println!("[calibration values] = {}", calibration_values);
    println!();

    println!("=== handle part two ===");
    let corrected_calibration_values: u32 = match part_two::run(&input_file) {
        Ok(corrected_calibration_values) => corrected_calibration_values,
        Err(e) => panic!("{}", e)
    };
    println!("[corrected calibration values] = {}", corrected_calibration_values);
    println!();
}
