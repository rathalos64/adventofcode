use std::env;
use std::path::Path;

mod point;
mod exercise;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let input: &String = &args.pop().unwrap_or_default();
    println!("> cwd {:?}, input {:?}", env::current_dir().unwrap_or_default(), input);
    assert!(Path::new(&input).exists());

    println!("=== part one (only horizontal and vertical lines) ===");
    let overlaps = match exercise::run(input, true) {
        Ok(overlaps) => overlaps,
        Err(e) => panic!("something went wrong: {:?}", e)
    };
    println!("number of line overlaps with n >= 2: {:?}", overlaps);
    println!();

    println!("=== part two (consider diagonal lines) ===");
    let overlaps = match exercise::run(input, false) {
        Ok(overlaps) => overlaps,
        Err(e) => panic!("something went wrong: {:?}", e)
    };
    println!("number of line overlaps with n >= 2: {:?}", overlaps);
}