use std::fs;
use std::collections::HashMap;

use crate::lantern;

pub fn run(input_path: String, days: u32) -> Result<lantern::LanternResult, String> {
    if days == 0 {
        return Err(String::from("days must be non-zero"));
    }

    let read: Vec<Vec<i32>> = fs::read_to_string(input_path)
        .unwrap_or_default()
        .split('\n')
        .filter(|line| *line != "")
        .map(|line| String::from(line)
            .split(",")
            .map(|state| String::from(state).parse::<i32>().unwrap()).collect()) // unwrap is not good
        .collect();
    if read.len() != 1 {
        return Err(String::from("expected exactly one lantern school to be read"));
    }

    let mut school: HashMap<i32, u64> = HashMap::new();
    for n in &read[0] {
        school.insert(*n, 1 + if school.contains_key(n) { school[n] } else { 0 });
    }
    for idx in 0 .. 9 { // fill the remaining states
        school.insert(idx, if school.contains_key(&idx) { school[&idx] } else { 0 });
    }

    let mut result: lantern::LanternResult = lantern::LanternResult::new();
    result.add(school.values().sum(), 0f64);

    for day in 0 .. days {
        let mut newborn: u64 = 0;
        
        for idx in 0 .. 8 {
            if idx == 0 {
                newborn = school[&0];
            }
            school.insert(idx, school[&(idx+1)]);
        }

        school.insert(6, newborn + school[&6]); // all that were newborn, were also reset
        school.insert(8, newborn); // for every fish, another is born

        let size: u64 = school.values().sum();
        let lambda: f64 = (size as f64 / result.fish[0] as f64).ln() / ((day + 1) as f64);
        result.add(size, lambda);
    }

    Ok(result)
}