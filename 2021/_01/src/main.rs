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
    println!("total number of increases: {:?}", part_one::run(&input));

    const SLIDING_SIZE: usize = 3;
    println!("=== handle part two ===");
    println!("total number of increases: {:?}", part_two::run(&input, SLIDING_SIZE));
}
