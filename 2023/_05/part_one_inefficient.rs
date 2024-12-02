use std::{fs::read_to_string, collections::HashMap};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

const SEEDS: &str = "seeds: ";

#[derive(Eq, PartialEq, Debug, Hash, EnumIter)]
pub enum AlmanacType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumitidy,
    HumidityToLocation,
}

pub fn run(input_file: &String) -> Result<i64, Box<dyn std::error::Error>> {
    let mut almanac: HashMap<AlmanacType, HashMap<i64, i64>> = HashMap::from([
        (AlmanacType::SeedToSoil, HashMap::new()),
        (AlmanacType::SoilToFertilizer, HashMap::new()),
        (AlmanacType::FertilizerToWater, HashMap::new()),
        (AlmanacType::WaterToLight, HashMap::new()),
        (AlmanacType::LightToTemperature, HashMap::new()),
        (AlmanacType::TemperatureToHumitidy, HashMap::new()),
        (AlmanacType::HumidityToLocation, HashMap::new()),
    ]);
    let find = HashMap::from([
        (AlmanacType::SeedToSoil, "seed-to-soil".to_owned()),
        (AlmanacType::SoilToFertilizer, "soil-to-fertilizer".to_owned()),
        (AlmanacType::FertilizerToWater, "fertilizer-to-water".to_owned()),
        (AlmanacType::WaterToLight, "water-to-light".to_owned()),
        (AlmanacType::LightToTemperature, "light-to-temperature".to_owned()),
        (AlmanacType::TemperatureToHumitidy, "temperature-to-humidity".to_owned()),
        (AlmanacType::HumidityToLocation, "humidity-to-location".to_owned()),
    ]);

    let body = read_to_string(input_file)?;
    let lines: Vec<&str> = body.lines().collect();
    assert_ne!(0, lines.len());

    // make seeds
    let seeds: Vec<i64> = lines[0].strip_prefix(SEEDS).unwrap()
        .split(" ")
        .map(|n| n.parse::<i64>().map_err(|x| x.to_string()))
        .collect::<Result<Vec<i64>, String>>()?;

    // make mapping
    let mut current: &mut HashMap<i64, i64> = &mut HashMap::default();
    for line in lines[2..].iter() {
        println!("{}", line);   
        if line.trim().is_empty() {
            continue
        }
        
        let matched = AlmanacType::iter() // a bit inefficient
            .filter(|at| line.contains(find.get(&at).unwrap()))
            .collect::<Vec<AlmanacType>>();
        if matched.len() > 0 {
            current = almanac
                .get_mut(matched.first().ok_or(std::fmt::Error)?)
                .ok_or(std::fmt::Error)?; // why fmt::Error?
                continue
        }
        
        let numbers = line.split(" ")
            .map(|n| n.parse::<i64>().map_err(|x| x.to_string()))
            .collect::<Result<Vec<i64>, String>>()?;
        assert_eq!(numbers.len(), 3);

        println!("until range collection");

        // dest start | source start | range length
        let source: Vec<i64> = (numbers[1]..(numbers[1]+numbers[2])).collect(); // source is 2nd
        let dest: Vec<i64> = (numbers[0]..(numbers[0]+numbers[2])).collect(); // dest is 1st
        assert_eq!(source.len(), dest.len());

        println!("until zipping");
        let mut entries: HashMap<i64, i64> = HashMap::new();
        for i in 0..source.len() {
            entries.insert(source[i], dest[i]);        
        }
        // make Vec<i64, i64> and make https://stackoverflow.com/a/30441736
        // to inefficient for big ranges
        // let entries: HashMap<i64, i64>
        //     = std::iter::zip(source, dest).collect::<Vec<(i64, i64)>>().into_iter().collect();
        
        (*current).extend(entries);
        println!("fin");
    }

    // find locations
    let mut locations: Vec<i64> = Vec::new();
    for seed in seeds.iter() {
        let mut current = seed.clone();
        for key in AlmanacType::iter() {
            let lookup = almanac.get(&key).ok_or(std::fmt::Error)?;
            println!("{:?} | {:?} go through", seed, key);
            current = *lookup.get(&current).unwrap_or(&current);
        }
        locations.push(current);
        println!();
    }
    println!("{:?}", locations);
    Ok(*locations.iter().min().ok_or(std::fmt::Error)?)

    // does not work (sad :( )
    // lines[2..].iter().try_for_each(|line| {
    //     for at in AlmanacType::iter() {
    //         current = almanac.get_mut(&at).ok_or(std::fmt::Error)?; // why fmt::Error?
    //     }
    //     Ok::<_, Box<dyn std::error::Error>>(())
    // });
}