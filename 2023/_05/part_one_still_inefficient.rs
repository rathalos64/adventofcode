use std::{fs::read_to_string, collections::HashMap};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

const SEEDS: &str = "seeds: ";

pub struct Mapping {
    source: i64,
    dest: i64,
    range: i64
}

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

pub fn run(input_file: &String) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let mut almanac: HashMap<AlmanacType, Vec<Mapping>> = HashMap::from([
        (AlmanacType::SeedToSoil, Vec::new()),
        (AlmanacType::SoilToFertilizer, Vec::new()),
        (AlmanacType::FertilizerToWater, Vec::new()),
        (AlmanacType::WaterToLight, Vec::new()),
        (AlmanacType::LightToTemperature, Vec::new()),
        (AlmanacType::TemperatureToHumitidy, Vec::new()),
        (AlmanacType::HumidityToLocation, Vec::new()),
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
    let raw_seeds: Vec<i64> = lines[0].strip_prefix(SEEDS).unwrap()
        .split(" ")
        .map(|n| n.parse::<i64>().map_err(|x| x.to_string()))
        .collect::<Result<Vec<i64>, String>>()?;
    let seeds: Vec<(i64, i64)> = raw_seeds.iter().map(|n| (*n, 1)).collect();

    // make mapping
    let mut current: &mut Vec<Mapping> = &mut Vec::default();
    for line in lines[2..].iter() {
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

        // dest start | source start | range length
        (*current).push(Mapping { source: numbers[1], dest: numbers[0], range: numbers[2] });
    }

    // find locations for all numbers being seeds
    let locations_all: Vec<i64> = find_locations(&seeds, &almanac)?;
    // println!("{:?}", locations_all);
    let min_locations_all = *locations_all.iter().min().ok_or(std::fmt::Error)?;

    // find locations for every tuple of seeds [(seed, range)]
    // https://stackoverflow.com/a/54420325
    let all_seeds_raw: Vec<(i64, i64)> = raw_seeds
        .chunks(2).map(|seed| (seed[0], seed[1])).collect::<Vec<(i64, i64)>>();
    let mut all_seeds: Vec<(i64, i64)> = Vec::new();
    for (source, range) in all_seeds_raw.iter() {
        let dest = source + range;
        let mut i: i64 = *source;
        println!("{}, {:?}", i, dest);
        while i < dest {
            all_seeds.push((i, 1));
            i = i + 1;
        }
    }
    println!("{:?}", all_seeds);
    let locations_tuple_seeds: Vec<i64> = find_locations(&all_seeds, &almanac)?;
    let min_locations_tuple = *locations_tuple_seeds.iter().min().ok_or(std::fmt::Error)?;

    Ok((min_locations_all, min_locations_tuple))
}

fn find_locations(seeds: &Vec<(i64, i64)>, almanac: &HashMap<AlmanacType, Vec<Mapping>>) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let mut locations: Vec<i64> = Vec::new();
    for seed_range in seeds.iter() {

        let mut seed = seed_range.0;
        let seed_dest = seed_range.0 + seed_range.1;
        println!("{:?}", seed_range);
        while seed < seed_dest {
            let mut current = seed.clone();

            for key in AlmanacType::iter() {
                let lookup = almanac.get(&key).ok_or(std::fmt::Error)?;

                // look through all mappings or keep the current one
                let mut found = current.clone();
                for mapping in lookup.iter() {
                    if mapping.source <= current && current <= (mapping.source + mapping.range) {
                        let offset = current - mapping.source;
                        found = mapping.dest + offset;
                        break
                    }
                }
                current = found;
            }

            locations.push(current);
            seed = seed + 1;
        }
    }

    Ok(locations)
}