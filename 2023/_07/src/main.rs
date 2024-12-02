use std::env;
use std::path::Path;

mod poker;
mod part_one;
mod part_two;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    assert!(args.len() > 1); // first is always the filename
    let input_file: String = args.pop().map_or_else(|| panic!("No arguments given"), |p| p);
    assert!(Path::new(&input_file).exists());

    println!("=== handle part one ===");
    let total_winnings: u64 = match part_one::run(&input_file) {
        Ok(result) => result,
        Err(e) => panic!("oh no {e}")
    };
    println!("[total_winnings] = {total_winnings}");
    println!();

    println!("=== handle part two ===");
    let total_winnings_with_joker: u64 = match part_two::run(&input_file) {
        Ok(result) => result,
        Err(e) => panic!("oh no {e}")
    };
    println!("[total_winnings_with_joker] = {total_winnings_with_joker}");
    println!();
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn part_one_full()  {
        let input = String::from("input");
        let r = part_one::run(&input);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 246795406);
    }

    #[test]
    fn part_two_full()  {
        let input = String::from("input");
        let r = part_two::run(&input);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 249356515);
    }
}
