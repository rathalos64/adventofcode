use std::env;
use std::path::Path;

mod part_one_two;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    assert!(args.len() > 1); // first is always the filename
    let input_file: String = match args.pop() {
        None => panic!("No arguments given"),
        Some(input_file) => input_file
    };
    assert!(Path::new(&input_file).exists());

    println!("=== handle part one/two ===");
    let (scratchcards_sum, nscratchcards): (i32, i32) = match part_one_two::run(&input_file) {
        Ok(result) => result,
        Err(e) => panic!("oh no {}", e)
    };
    println!("[scratchcards_sum old rule] = {}", scratchcards_sum);
    println!("[nscratchcards] = {}", nscratchcards);
    println!();
}
