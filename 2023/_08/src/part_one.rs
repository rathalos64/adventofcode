use std::fs::read_to_string;
use crate::wasteland::*;

pub fn run(input_file: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let body = read_to_string(input_file)?;
    let wasteland = Wasteland::try_from(String::from(body))?;
    Ok(solve(&wasteland)?)
}

fn solve(wasteland: &Wasteland) -> Result<u64, WastelandError> {
    let start_key: Key = Key::try_from("AAA")?;
    let end_key: Key = Key::try_from("ZZZ")?;

    let (instructions, network) = (&wasteland.instructions, &wasteland.network);
    let mut current = start_key;
    let mut steps = 0;
    for i in 0..10_000_000 { // max allowed loop
        current = network.get(&current)
            .map_or_else(|| Err(WastelandError(format!("could not find {current}"))), |v| Ok(v.clone()))?
            .get_direction(&instructions[i%instructions.len()]);
        steps += 1;

        if current == end_key {
            return Ok(steps)
        }
    }

    Err(WastelandError("failed to find a solution, running in circles".to_string()))
}