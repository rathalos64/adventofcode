use std::fs;
use std::env;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
enum ParseError {
    EOF,
    Corrupted(char),
    Incomplete(Vec<char>)
}

impl ParseError {
    fn __str(&self) -> String {
        match self {
            ParseError::EOF => String::from("EOF"),
            ParseError::Corrupted(c) => format!("corrupted char {}", c),
            ParseError::Incomplete(missing) => format!("incomplete, missing {}", missing.iter().collect::<String>())
        }
    }

    fn score(&self) -> u64 {
        let illegal: HashMap<char, u64> = HashMap::from([
            (')', 3),
            (']', 57),
            ('}', 1197),
            ('>', 25137)
        ]);

        let incomplete: HashMap<char, u64> = HashMap::from([
            (')', 1),
            (']', 2),
            ('}', 3),
            ('>', 4)
        ]);

        match self {
            ParseError::EOF => 0,
            ParseError::Corrupted(c) => illegal[c],
            ParseError::Incomplete(missing) => missing.iter().fold(0, |acc, curr| acc * 5 + incomplete[curr])
        }
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let input: &String = &args.pop().unwrap_or_default();
    println!("> cwd {:?}, input {:?}", env::current_dir().unwrap_or_default(), input);
    assert!(Path::new(&input).exists());

    let lines: Vec<String> = fs::read_to_string(input).unwrap_or("".into()).split('\n')
        .filter(|line| *line != "")
        .map(|line| line.into())
        .collect();
    assert_ne!(lines.len(), 0);

    println!("=== [ obtain syntax scorings (illegal, incomplete) ] ====================");

    let mut illegal: u64 = 0;
    let mut incomplete: Vec<u64> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        match parse(&line.chars().collect(), 0) {
            Ok(_) | Err(ParseError::EOF) => println!("{}:{} OK", input, i),
            Err(e) => {
                println!("{}:{} invalid line: {}", input, i, e.__str());
                match e {
                    ParseError::Corrupted(_) => { illegal += e.score(); }
                    ParseError::Incomplete(_) => { incomplete.push(e.score()); }
                    _ => {}
                }
            }
        }
    }

    println!();
    println!("[part one] illegal score: \t{}", illegal);
    println!("[part two] incomplete score: \t{}", middle(incomplete) as usize);
}

fn parse(tokens: &Vec<char>, pos: i32) -> Result<i32, ParseError> {
    let curr: &char = match tokens.get(pos as usize) {
        Some(curr) => curr,
        _ => return Err(ParseError::EOF)
    };

    match curr {
        '(' | '[' | '{' | '<' => {
            match parse(tokens, pos+1) {
                Ok(pos) => {
                    let next: &char = tokens.get(pos as usize).unwrap();
                    if *curr != matching(next) {
                        return Err(ParseError::Corrupted(*next));
                    }
                    return parse(tokens, pos+1); // continue
                },
                Err(ParseError::EOF) => {
                    return Err(ParseError::Incomplete(vec![matching(curr)])) // EOF with open paranthesis
                }
                Err(ParseError::Incomplete(mut missing)) => { // extend incomplete to upper layer
                    missing.push(matching(curr));
                    return Err(ParseError::Incomplete(missing));
                }
                Err(e) => return Err(e) // propagate error (bit weird)
            }
        }
        ')' | ']' | '}' | '>' => {
            return Ok(pos)
        }
        _ => panic!("no parenthesis given")
    }
}

fn middle(mut list: Vec<u64>) -> u64 {
    assert_ne!(list.len(), 0);

    list.sort(); // inefficient to copy whole array but let's stick with it
    list[list.len() / 2]
}

fn matching(c: &char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("should have been a parenthesis!"),
    }
}
