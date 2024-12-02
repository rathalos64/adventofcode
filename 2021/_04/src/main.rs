mod bingo;

use std::fs;
use std::env;
use std::path::Path;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let input: &String = &args.pop().unwrap_or_default();
    println!("> cwd {:?}, input {:?}", env::current_dir().unwrap_or_default(), input);
    assert!(Path::new(&input).exists());

    println!("=== read input ===");
    let to_draw = match read_draw_numbers(String::from(input)) {
        Ok(to_draw) => to_draw,
        Err(err) => panic!("failed to read line numbers from {:?}: {:?}", input, err)
    };
    assert_ne!(0, to_draw.len());

    println!("=== let's play some bingo ===");
    let mut game = match bingo::Game::init(String::from(input)) {
        Ok(game) => game,
        Err(err) => panic!("failed to initialize bingo game: {}", err)
    };

    let mut won_boards: Vec<(usize, i32)> = Vec::new();
    for draw in to_draw {
        game.draw(draw);

        if let Some(mut winners) = game.check_winner() {
            won_boards.append(&mut winners);
        }
    }
    assert_ne!(won_boards.len(), 0);

    println!("=== analyse winners (squid vs you) ===");
    let first = won_boards.first().unwrap();
    let last = won_boards.last().unwrap();
    println!("> first winner (#, score): ({:?}, {:?})", first.0, first.1);
    println!("> last winner  (#, score): ({:?}, {:?})", last.0, last.1);
}

fn read_draw_numbers(input_path: String) -> std::io::Result<Vec<i32>> {
    let file = fs::File::open(input_path);
    let mut buffer = BufReader::new(file?);

    let mut draw_line = String::new();
    if let Err(res) = buffer.read_line(&mut draw_line) {
        return Err(res)
    }   

    // assume first line are the numbers
    let to_draw: Vec<i32> = draw_line.trim_end().split(",")
        .filter(|n| *n != "")
        .map(|n| String::from(n).parse::<i32>().unwrap())
        .collect();

    Ok(to_draw)
}
