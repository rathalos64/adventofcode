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
    let sub_part_one = part_one::run(&input);
    println!("[horizontal, depth] = [{}, {}]", sub_part_one.horizontal(), sub_part_one.depth());
    println!("[multiplied] = {}", sub_part_one.horizontal() * sub_part_one.depth());

    println!();
    println!("=== handle part two ===");
    let sub_part_two = part_two::run(&input);
    println!("[horizontal, depth] = [{}, {}]", sub_part_two.horizontal(), sub_part_two.depth());
    println!("[multiplied] = {}", sub_part_two.horizontal() * sub_part_two.depth());
}
