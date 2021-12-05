use std::fs;
use std::collections::HashMap;

use crate::point;

pub fn run(input_path: &String, only_vertical_horizontal: bool) -> Result<usize, String> {
    let read: Vec<Vec<point::Point>> = fs::read_to_string(input_path)
        .unwrap_or_default()
        .split('\n')
        .filter(|line| *line != "")
        .map(|line| String::from(line).split(" -> ")
            .map(|e| point::Point::new_from_string(e).unwrap()).collect()) // unwrap is not good
        .collect();
    if read.len() == 0 {
        return Err(String::from("no points were read"));
    }

    let mut points: Vec<(point::Point, point::Point)> = Vec::new();
    for tuple in read {
        if tuple.len() != 2 {
            return Err(String::from("invalid number of points per line"));
        }
        if tuple[0].x != tuple[1].x && tuple[0].y != tuple[1].y && only_vertical_horizontal {
            continue
        }

        points.push((tuple[0], tuple[1]));
    }

    let mut diagram: HashMap<String, i32> = HashMap::new();
    for (from, to) in points {
        let line = match point::get_line(from, to, None) {
            Ok(line) => line,
            Err(_) => return Err(String::from("failed to get line"))
        };

        for point in line {
            let occurrence = diagram.entry(point.str()).or_insert(0); 
            *occurrence += 1;
        }
    }

    let overlaps: Vec<&i32> = diagram.values().filter(|v| **v > 1).collect(); // what the double pointer?!
    Ok(overlaps.len())
}