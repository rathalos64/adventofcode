use std::env;
use std::path::Path;

mod part;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    assert!(args.len() > 1); // first is always the filename
    let input_file: String = match args.pop() {
        None => panic!("No arguments given"),
        Some(input_file) => input_file
    };
    assert!(Path::new(&input_file).exists());

    println!("=== handle part one ===");
    let (record_beat_all_races, record_beat_summed_races): (i64, i64) = match part::run(&input_file) {
        Ok(result) => result,
        Err(e) => panic!("oh no {}", e)
    };
    println!("[record_beat_all_races] = {}", record_beat_all_races);
    println!("[record_beat_summed_races] = {}", record_beat_summed_races);
    println!();
}
