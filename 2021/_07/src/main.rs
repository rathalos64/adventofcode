use std::fs;
use std::env;
use std::path::Path;
use std::io::{prelude::*, BufReader};

mod calc;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let input: &String = &args.pop().unwrap_or_default();
    println!("> cwd {:?}, input {:?}", env::current_dir().unwrap_or_default(), input);
    assert!(Path::new(&input).exists());

    // read input
    let mut X = match read_draw_numbers(String::from(input)) {
        Ok(X) => X,
        Err(err) => panic!("failed to read line numbers from {:?}: {:?}", input, err)
    };
    X.sort();
    assert_ne!(0, X.len());

    let start = *X.iter().min().unwrap_or(&0) as f64;
    
    println!("=== [using gradient descent] ===================");
    println!("=== [part one] =================================");
    let base = |pos: f64| X.iter()
        .map(|x| *x as f64)
        .map(|x| (x - pos).abs()).sum();
    let derivate = |pos: f64| X.iter()
        .map(|x| *x as f64)
        .map(|x| calc::inf_nan_or((x - pos) / ((x - pos).abs()), 0.0)).sum::<f64>() * -1.0;
    let part_one: calc::Function = calc::Function {
        base: &base,
        derivate: Some(&derivate)
    };

    let (x_best, epochs) = calc::gradient_descent(start, part_one, None);
    println!("[epochs = {}] x_best = {}, fuel = {}", epochs, x_best, (part_one.base)(x_best as f64));
    println!();

    println!("=== [part two] =================================");
    let base = |pos: f64| X.iter()
        .map(|x| *x as f64)
        .map(|x| ((x - pos).powf(2.0) + (x - pos).abs()) / 2.0).sum();
    let derivate = |pos: f64| X.iter()
        .map(|x| *x as f64)
        .map(|x| calc::inf_nan_or((2.0 * (x - pos) + 1.0) / 2.0, 0.0)).sum::<f64>() * -1.0;
    let part_two: calc::Function = calc::Function {
        base: &base,
        derivate: Some(&derivate)
    };

    let params: calc::Options = calc::Options{
        alpha: Some(0.0001), 
        decay: None, 
        max_tries: None, 
        max_reps: None
    };

    let (x_best, epochs) = calc::gradient_descent(start, part_two, Some(params));
    println!("[epochs = {}] x_best = {}, fuel = {}", epochs, x_best, (part_two.base)(x_best as f64));
    println!();

    println!("=== [using your brain] =================================");
    let median: i32 = calc::median(&X) as i32;
    let mean: i32 = calc::mean(&X) as i32;

    println!("[part_one] median = {}, fuel = {}", median, (part_one.base)(median as f64));
    println!("[part_two] mean   = {}, fuel = {}", mean, (part_two.base)(mean as f64));
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