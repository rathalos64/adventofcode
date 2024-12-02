mod lantern;
mod part_one;
mod part_two;

use std::fs;
use std::env;
use std::path::Path;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    println!("> cwd {:?}, input {:?}", env::current_dir().unwrap_or_default(), args);

    let days: u32 = match args.pop() {
        Some(input) => match String::from(input).parse::<u32>() {
            Ok(days) => days,
            Err(_) => panic!("failed to parse days")
        }
        None => panic!("no input given"),
    };
    assert_ne!(days, 0);

    let input: String = match args.pop() {
        Some(input) => input,
        None => panic!("no input given"),
    };
    assert!(Path::new(&input).exists());

    // println!("=== part one (better implementation but awful performance) ===");
    // let result = match part_one::run(input.clone(), days) {
    //     Ok(result) => result,
    //     Err(e) => panic!("[part_one] something went wrong: {:?}", e)
    // };
    // assert_ne!(result.fish.len(), 0); assert_ne!(result.lambdas.len(), 0);
    // println!("initial state: {}, $l$ = {}", result.fish[0], result.lambdas[0]);
    // println!("after {} days: {}, $l$ = {}", days, result.fish.last().unwrap(), result.lambdas.last().unwrap());
    // println!();

    println!("=== part two (more hacky implementation but brutally fast) ===");
    let result = match part_two::run(input.clone(), days) {
        Ok(result) => result,
        Err(e) => panic!("[part_two] something went wrong: {:?}", e)
    };
    assert_ne!(result.fish.len(), 0); assert_ne!(result.lambdas.len(), 0);
    println!("initial state: {}, $l$ = {}", result.fish[0], result.lambdas[0]);
    println!("after {} days: {}, $l$ = {}", days, result.fish.last().unwrap(), result.lambdas.last().unwrap());
    println!();

    // [ expiremental phase ] - model the data with simple one-coeff exponential function
    let mut fish_fit: Vec<u64> = Vec::new();
    let fish0: f64 = result.fish[0] as f64;
    let lambda: f64 = *result.lambdas.last().unwrap(); // always take the last calculated lambda (higher precision)
    for day in 0 .. days+1 {
        fish_fit.push((fish0 * (day as f64 * lambda).exp()) as u64); // y = M * exp(t * lambda)
    }

    for i in 0..fish_fit.len() {
        println!("[#{}][fish, fish_fit] = [{}, {}]", i, result.fish[i], fish_fit[i]);
    }

    // calculate the fitting coefficient R^2 (run with --release flag or overflow err)
    let r2: f64 = r_squared(result.fish, fish_fit);
    println!("[modelling] fitting the fish growth with a one-coeff exponential function: $R^2$ = {:?}", r2)
}

fn r_squared(fish: Vec<u64>, fish_fit: Vec<u64>) -> f64 {
    assert_eq!(fish.len(), fish_fit.len());
    
    let mean: f64 = (fish.iter().sum::<u64>() as f64) / (fish.len() as f64);

    // calculate sum of squares {residual, total}
    let ss_res: f64 = (0..fish.len()).map(|i| (fish[i] - fish_fit[i]).pow(2)).sum::<u64>() as f64;
    let ss_tot: f64 = (0..fish.len()).map(|i| (fish[i] as f64 - mean).powf(2f64)).sum::<f64>();

    1f64 - (ss_res / ss_tot)
}
