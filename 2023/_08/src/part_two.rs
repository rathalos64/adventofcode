use std::fs::read_to_string;
use rayon::prelude::*;
use num::integer::lcm;
use crate::wasteland::*;

pub fn run(input_file: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let body = read_to_string(input_file)?;
    let wasteland = Wasteland::try_from(String::from(body))?;
    Ok(solve(&wasteland)?)
}

fn solve(wasteland: &Wasteland) -> Result<u64, WastelandError> {
    let (instructions, network) = (&wasteland.instructions, &wasteland.network);
    let starts: Vec<Key> = network // you are a ghost, find all start locations
        .keys().filter(|k| k.key.ends_with("A"))
        .map(|k| k.clone()).collect();

    // instead of continuing the simulation forever, take the lenghts
    // of each run per start location 
    // kudos to: https://blog.oziomaogbe.com/2024/01/09/what-i-learned-from-solving-2023-advent-of-code-puzzle.html
    let durations = starts.par_iter().map(|key| {
        let mut current = key.clone();
        let mut steps = 0;
        
        for i in 0..10_000_000 { // max allowed loop
            current = network.get(&current)
                .map_or_else(|| Err(WastelandError(format!("could not find {current}"))), |v| Ok(v.clone()))?
                .get_direction(&instructions[i%instructions.len()]);
            steps += 1;

            if current.key.ends_with("Z") { // end locations have "Z" at the end
                return Ok(steps)
            }
        }

        Err(WastelandError(format!("failed to find a solution for start {key}")))
    }).collect::<Result<Vec<u64>, WastelandError>>()?;
    
    // compute the steps for each start to end of Z 
    // by taking the least common multiple (lcm) of the lengths
    // 1 is a great start
    //
    // reminder: 
    // lcm(a, b) is a*b / gcd(a, b) (greatest common divisor)
    // lcm(6, 8) = 48 / 2 = 24
    // https://docs.rs/num/latest/num/integer/fn.lcm.html
    let mut overlapping = 1;
    for duration in durations.into_iter() {
        overlapping = lcm(overlapping, duration)
    }
    
    Ok(overlapping)
}