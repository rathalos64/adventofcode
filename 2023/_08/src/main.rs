use std::env;
use std::path::Path;

mod wasteland;
mod part_one;
mod part_two;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    assert!(args.len() > 1); // first is always the filename
    let input_file: String = args.pop().map_or_else(|| panic!("No arguments given"), |p| p);
    assert!(Path::new(&input_file).exists());

    println!("=== handle part one ===");
    let steps_required: u64 = match part_one::run(&input_file) {
        Ok(result) => result,
        Err(e) => panic!("oh no {e}")
    };
    println!("[steps_required] = {steps_required}");

    println!("=== handle part two ===");
    let steps_required_ghost: u64 = match part_two::run(&input_file) {
        Ok(result) => result,
        Err(e) => panic!("oh no {e}")
    };
    println!("[steps_required_ghost] = {steps_required_ghost}");
    println!();
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn part_one_1()  {
        let input = String::from("part_one_input.test.1");
        let r = part_one::run(&input);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 2);
    }

    #[test]
    fn part_one_2()  {
        let input = String::from("part_one_input.test.2");
        let r = part_one::run(&input);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 6);
    }

    #[test]
    fn part_one_full()  {
        let input = String::from("input");
        let r = part_one::run(&input);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 19241);
    }

    #[test]
    fn part_two_1()  {
        let input = String::from("part_two_input.test.1");
        let r = part_two::run(&input);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 6);
    }
}
