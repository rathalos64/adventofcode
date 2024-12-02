use std::fs;
use crate::lantern;

pub fn run(input_path: String, days: u32) -> Result<lantern::LanternResult, String> {
    if days == 0 {
        return Err(String::from("days must be non-zero"));
    }
    if days > 130 {
        return Err(String::from("due to the inefficiency of the algorithm more than 130 iterations cannot be made"));
    }

    let read: Vec<lantern::LanternSchool> = fs::read_to_string(input_path)
        .unwrap_or_default()
        .split('\n')
        .filter(|line| *line != "")
        .map(|line| lantern::LanternSchool::from_iter(String::from(line)
            .split(",")
            .map(|state| lantern::LanternFish::new_from_string(state).unwrap()))) // unwrap is not good
        .collect();
    if read.len() != 1 {
        return Err(String::from("expected exactly one lantern school to be read"));
    }

    let mut school: lantern::LanternSchool = read[0].clone();
    let mut result: lantern::LanternResult = lantern::LanternResult::new();
    result.add(school.size() as u64, 0f64);

    for day in 0..days {
        school.pass_day();

        let size: u64 = school.size() as u64;
        let lambda: f64 = (size as f64 / result.fish[0] as f64).ln() / ((day + 1) as f64);
        result.add(size, lambda);
    }

    Ok(result)
}