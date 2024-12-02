use std::fs::read_to_string;
use crate::token::Grid;

pub fn run(input_file: &String) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let body = read_to_string(input_file)?;
    let lines: Vec<String> = body.lines().map(String::from).collect();
    assert_ne!(0, lines.len());

    let mut grid: Grid = Grid::default();
    for lin in lines.iter() {
        let chars: Vec<String> = lin.chars().map(String::from).collect();
        grid.append(&chars)?;
    }
    grid.populate()?;

    let mut sum: i32 = 0;
    let mut sum_ratios: i32 = 0;
    for token in grid.tokens.iter() {
        if token.is_part_number() {
            let part_number = token.get_part_number()?;
            sum = sum + part_number;
        }
        if token.is_gear() {
            let ratio = match token.get_gear_ratio() {
                Ok(ratio) => ratio,
                Err(e) => panic!("{}", e)
            };
            sum_ratios = sum_ratios + ratio;
        }
    }

    Ok((sum, sum_ratios))
}
