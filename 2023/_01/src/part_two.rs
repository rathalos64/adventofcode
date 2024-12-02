use std::fs::read_to_string;
use std::error::Error;
use std::collections::HashMap;
use std::convert::TryFrom;

pub fn run(input_file: &String) -> Result<u32, Box<dyn Error>> {
    let body = match read_to_string(input_file) {
        Ok(body) => body,
        Err(e) => return Err(Box::new(e))
    };
    let lines: Vec<String> = body.lines().map(String::from).collect();
    assert_ne!(0, lines.len());

    let mut sum: u32 = 0;
    let words: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    for line in lines {
        let mut indices: Vec<(i32, u32)> = line.chars()
            .enumerate()
            .filter(|&(_, c)| c.is_numeric())
            .map(|(i, c)| (i32::try_from(i).ok().unwrap(), String::from(c).parse::<u32>().ok().unwrap()))
            .collect();

        let mut word_indices = words.keys()
            .map(|key| (line.match_indices(key), words[key]))
            .map(|tup| tup.0.map(move |index| (i32::try_from(index.0).ok().unwrap(), tup.1)))
            .flatten()
            .collect();

        indices.append(&mut word_indices);
        indices.sort_by(|a, b| a.0.cmp(&b.0));
        if indices.len() == 0 {
            continue
        }

        let first = indices.first().unwrap().1;
        let last = indices.last().unwrap().1;
        let number = format!("{}{}", first, last);
        sum = sum + match number.parse::<u32>() {
            Ok(i) => i,
            Err(e) => return Err(Box::new(e))
        };
    }

    return Ok(sum)
}