use std::fs::read_to_string;
use std::error::Error;

pub fn run(input_file: &String) -> Result<u32, Box<dyn Error>> {
    let body = match read_to_string(input_file) {
        Ok(body) => body,
        Err(e) => return Err(Box::new(e))
    };
    let lines: Vec<String> = body.lines().map(String::from).collect();
    assert_ne!(0, lines.len());

    let mut powers_sum: u32 = 0;
    for line in lines {
        let id_sets: Vec<&str> = line.split(":").collect();
        assert_eq!(id_sets.len(), 2);

        let mut reds: Vec<u32> = vec![];
        let mut greens: Vec<u32> = vec![];
        let mut blues: Vec<u32> = vec![];
        for game_set in id_sets[1].split(";").collect::<Vec<&str>>() {
            for cube in game_set.trim().split(",").collect::<Vec<&str>>() {
                let count_color = cube.trim().split_whitespace().collect::<Vec<&str>>();
                assert_eq!(count_color.len(), 2);

                let count: u32 = match count_color[0].parse::<u32>() {
                    Ok(i) => i,
                    Err(e) => return Err(Box::new(e))
                };
                let color: &str = count_color[1];
                
                match color {
                    "red" => reds.push(count),
                    "green" => greens.push(count),
                    "blue" => blues.push(count),
                    _ => ()
                };
            }
        }
        reds.sort();
        greens.sort();
        blues.sort();

        let power = reds.last().unwrap() * greens.last().unwrap() * blues.last().unwrap();
        println!("{}", power);
        powers_sum += power;
    }

    return Ok(powers_sum)
}