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
    let valid_games: u32 = match part_one::run(&input_file) {
        Ok(valid_games) => valid_games,
        Err(e) => panic!("{}", e)
    };
    println!("[valid_games] = {}", valid_games);
    println!();

    println!("=== handle part two ===");
    let sum_of_powers: u32 = match part_two::run(&input_file) {
        Ok(sum_of_powers) => sum_of_powers,
        Err(e) => panic!("{}", e)
    };
    println!("[sum_of_powers] = {}", sum_of_powers);
    println!();
}
