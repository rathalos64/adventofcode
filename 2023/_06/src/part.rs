use std::fs::read_to_string;
use std::ops::RangeInclusive;
use rayon::prelude::*;

pub fn run(input_file: &String) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let body = read_to_string(input_file)?;
    let lines: Vec<&str> = body.lines().collect();
    assert_eq!(lines.len(), 2);

    let mut races: Vec<i64> = Vec::new();
    let mut record: Vec<i64> = Vec::new();
    for line in lines.iter() {
        let split: Vec<&str> = line.split(":").map(|x| x.trim()).collect();
        assert_eq!(split.len(), 2);
        let (prefix, numberss): (&str, &str) = (split[0].trim(), split[1].trim());

        let numbers_split = numberss.split(" ").map(|x| x.trim()).filter(|&x| x != "").collect::<Vec<&str>>();
        match prefix {
            "Time" => races = numbers_split.iter().map(|x| x.trim().parse::<i64>().map_err(|x| x.to_string())).collect::<Result<Vec<i64>, String>>()?,
            "Distance" => record = numbers_split.iter().map(|x| x.trim().parse::<i64>().map_err(|x| x.to_string())).collect::<Result<Vec<i64>, String>>()?,
            _ => panic!("nooo")
        }
    }
    assert_eq!(races.len(), record.len());

    let successes_all_races = part_one_all_races(&races, &record)?;
    let successes_summed_races = part_two_summed_races(
        races.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("").parse::<i64>()?, 
        record.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("").parse::<i64>()?
    );
    Ok((successes_all_races, successes_summed_races))
}

fn part_one_all_races(races: &Vec<i64>, record: &Vec<i64>) -> Result<i64, Box<dyn std::error::Error>> {
    let successes_per_race: Vec<i64> = races.iter().enumerate().map(|(i, race_duration)| {
        find_successes(*race_duration, record[i])}).collect();
    Ok(successes_per_race.into_iter().reduce(|acc, curr| acc * curr).ok_or(std::fmt::Error)?)
}

fn part_two_summed_races(race_duration: i64, record: i64) -> i64 {
    find_successes(race_duration, record)
}

fn find_successes(race_duration: i64, record: i64) -> i64 {
    // leverage symmetry for that problem to increase performance
    // 
    // special cases to not consider:
    // - 0 seconds => will yield 0 millimetres
    // - entire duration => will yield 0 millimetres
    if race_duration % 2 == 1 {
        // for 7 => 1 2 3 (| 3 4 5 |)
        evaluate_parallel(1..=((race_duration - 1) / 2), race_duration, record) * 2
    } else {
        // for 8 => (1 2 3) + (4 5 6 7)
        let middle = race_duration - 2;
        evaluate_parallel(1..=(middle / 2), race_duration, record) + 
            evaluate_parallel((race_duration / 2)..=(race_duration - 1), race_duration, record)
    }
}

// old: evaluate sequentially
// fn evaluate(range: RangeInclusive<i64>, race_duration: i64, record: i64) -> i64 {
//     range.map(|how_long| hold_button(race_duration, how_long))
//         .filter(|&traveled| traveled > record)
//         .collect::<Vec<i64>>().len() as i64
// }

// evaluate concurrently (https://kerkour.com/rust-worker-pool)
// from rayon crate https://docs.rs/rayon/latest/rayon/
// the trait https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html
fn evaluate_parallel(range: RangeInclusive<i64>, race_duration: i64, record: i64) -> i64 {
    range.into_par_iter()
        .map(|how_long| hold_button(race_duration, how_long))
        .filter(|&traveled| traveled > record)
        .collect::<Vec<i64>>().len() as i64
}

fn hold_button(race_duration: i64, how_long: i64) -> i64 {
    assert!(how_long <= race_duration);
    (race_duration - how_long) * how_long
}