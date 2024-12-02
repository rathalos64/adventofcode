use std::fs::read_to_string;
use std::error::Error;

pub fn run(input_file: &String) -> Result<u32, Box<dyn Error>> {
    let body = match read_to_string(input_file) {
        Ok(body) => body,
        Err(e) => return Err(Box::new(e))
    };
    let lines: Vec<String> = body.lines().map(String::from).collect();
    assert_ne!(0, lines.len());

    let mut sum: u32 = 0;
    for line in lines {
        let s: String = match line.chars().filter(|c| c.is_numeric())
            .map(String::from)
            .reduce(|cur: String, nxt: String| cur + &nxt) {
                Some(s) => s,
                None => String::from("0")
            };
        let chars: Vec<char> = s.chars().collect();
        let first = chars.first().unwrap().to_string();
        let last = chars.last().unwrap().to_string();
        let number = format!("{}{}", first, last);

        sum = sum + match number.parse::<u32>() {
            Ok(i) => i,
            Err(e) => return Err(Box::new(e))
        };
    }

    return Ok(sum)
}