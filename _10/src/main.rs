use std::fs;
use std::env;
use std::path::Path;
use std::collections::HashMap;

const END_EOF: i32        = -1;
const END_CORRUPTED: i32  = -2;
const END_INCOMPLETE: i32 = -3;

enum ParseError {
    EOF,
    END_CORRUPTED(char), // error char
    END_INCOMPLETE
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

    let errorScores: HashMap<char, usize> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);

    let error: usize = 0;
    for (i, line) in lines.iter().enumerate() {
        let status: i32 = parse(&line.chars().collect(), 0);
        if status <= 0 {
            println!("[{}] error with status {}", i, status);
            continue
        }

        println!("[{}] OK", i)
    }
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
                    let close: &char = tokens.get(pos as usize).unwrap();
                    if *curr != matching(close) {
                        return Err(ParseError::END_CORRUPTED(*close));
                    }
                    return parse(tokens, pos+1);
                },
                Err(ParseError::EOF) => {
                    return Err(ParseError::END_INCOMPLETE) // did not expect EOF after open parenthesis
                }
                Err(e) => return Err(e) // propagate error
            }
        }
        ')' | ']' | '}' | '>' => {
            return Ok(pos)
        }
        _ => panic!("no parenthesis given")
    }
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
