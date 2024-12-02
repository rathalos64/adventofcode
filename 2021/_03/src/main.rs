mod part_one;
mod part_two;

use std::env;
use std::path::Path;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let input: &String = &args.pop().unwrap_or_default();
    println!("> Binary dir {:?}, input {:?}", env::current_dir().unwrap_or_default(), input);
    assert!(Path::new(&input).exists());

    println!("=== handle part one ===");
    let power_consumption = part_one::run(input);
    println!("[power consumption] = {}", power_consumption);
    println!();

    println!("=== handle part two ===");
    let life_support_rating = part_two::run(input);
    println!("[power life_support_rating] = {}", life_support_rating);
    println!()
}
