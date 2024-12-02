use std::env;
use std::path::Path;

mod part_one;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    assert!(args.len() > 1); // first is always the filename
    let input_file: String = match args.pop() {
        None => panic!("No arguments given"),
        Some(input_file) => input_file
    };
    assert!(Path::new(&input_file).exists());

    println!("=== handle part one ===");
    let (min_location_all, min_location_tuple): (i64, i64) = match part_one::run(&input_file) {
        Ok(result) => result,
        Err(e) => panic!("oh no {}", e)
    };
    println!("[min_location_all] = {}", min_location_all);
    println!("[min_location_tuple] = {}", min_location_tuple);
    println!();
}
