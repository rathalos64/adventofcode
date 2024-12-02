use std::{fs::read_to_string, collections::HashMap};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::cmp;

const SEEDS: &str = "seeds: ";

#[derive(Debug)]
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

// #1
// src: 50 .. 98
// dst: 52 .. 100
// 
// src (79 - 50, 92 - 98) = (29, -6)
// dst (52 + 29, 100 + -6) = (81, 94)
pub fn run(input_file: &String) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    test_map_ranges();
    return Ok((1, 2));

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

    // find locations for every tuple of seeds [(seed, range)], convert to [(from, to)]
    // https://stackoverflow.com/a/54420325
    let all_seeds_raw: Vec<(i64, i64)> = raw_seeds
        .chunks(2).map(|seed| (seed[0], seed[0] + (seed[1] - 1))).collect::<Vec<(i64, i64)>>();

    let mut min_locations: Vec<(i64, i64)> = Vec::new();
    for seed in all_seeds_raw.iter() {
        let ranges = find_locations(*seed, &almanac)?;
        let min = ranges.iter().min_by_key(|x| x.0).ok_or(std::fmt::Error)?;
        min_locations.push(*min);
        // println!("seed {:?} | min {:?} | ranges {:?}", seed, min, ranges);
    }

    println!("over all mins: {:?}", min_locations);
    let over_all_min = min_locations.iter().min_by_key(|x| x.0).ok_or(std::fmt::Error)?;

    // // find locations for all numbers being seeds
    // let locations_all: Vec<i64> = find_locations(&seeds, &almanac)?;
    // // println!("{:?}", locations_all);
    // let min_locations_all = *locations_all.iter().min().ok_or(std::fmt::Error)?;

    // // find locations for every tuple of seeds [(seed, range)]
    // // https://stackoverflow.com/a/54420325
    // let all_seeds_raw: Vec<(i64, i64)> = raw_seeds
    //     .chunks(2).map(|seed| (seed[0], seed[1])).collect::<Vec<(i64, i64)>>();
    // let mut all_seeds: Vec<(i64, i64)> = Vec::new();
    // for (source, range) in all_seeds_raw.iter() {
    //     let dest = source + range;
    //     let mut i: i64 = *source;
    //     println!("{}, {:?}", i, dest);
    //     while i < dest {
    //         all_seeds.push((i, 1));
    //         i = i + 1;
    //     }
    // }
    // println!("{:?}", all_seeds);
    // let locations_tuple_seeds: Vec<i64> = find_locations(&all_seeds, &almanac)?;
    // let min_locations_tuple = *locations_tuple_seeds.iter().min().ok_or(std::fmt::Error)?;

    Ok((2, over_all_min.0))
}

fn find_locations(seed: (i64, i64), almanac: &HashMap<AlmanacType, Vec<Mapping>>) -> Result<Vec<(i64, i64)>, Box<dyn std::error::Error>> {
    // let mut locations: Vec<i64> = Vec::new();
    let mut ranges: Vec<(i64, i64)> = vec![seed];
    
    for key in AlmanacType::iter() {
        let mappings = almanac.get(&key).ok_or(std::fmt::Error)?;
        println!("{:?} | ranges {:?}", key, ranges);

        // iterate over all sub ranges
        let length = ranges.len();
        let mut range_idx = 0;
        while range_idx < length { // prevent loop getting bigger if extending with sub ranges
            let mut left_overs = vec![ranges[range_idx]];
            let mut converted: Vec<(i64, i64)> = vec![];
            // println!("range idx {}", range_idx);

            // determine sub ranges
            // left overs are ranges to be evaluated
            for mapping in mappings.iter() {
                // print!("evaluate mapping {:?} - ", mapping);
                
                let mut lefto_idx: i32 = 0;
                while lefto_idx < left_overs.len() as i32 {
                    let (from, to) = left_overs[lefto_idx as usize];
                    let (mapping_from, mapping_to) = (mapping.source, mapping.source + mapping.range);
                    
                    // must be in range, at least partially
                    if (from <= mapping_from && to <= mapping_from) ||
                        (from >= mapping_to && to >= mapping_to) {
                            // println!("does not apply");
                            break
                    }
                    // println!("applies");

                    // map matching range to dest range
                    let (diff_from, diff_to) = (from - mapping_from, to - mapping_to);
                    // left_overs[lefto_idx] = (
                    //     mapping.dest + cmp::max(diff_from, 0),
                    //     (mapping.dest + mapping.range) + cmp::min(diff_to, 0)
                    // );
                    converted.push((
                        mapping.dest + cmp::max(diff_from, 0),
                        (mapping.dest + mapping.range) + cmp::min(diff_to, 0)
                    ));
                    left_overs.remove(lefto_idx as usize); // remove found from left_overs
                    lefto_idx = lefto_idx - 1;

                    // determine upper bound / lower bound sub ranges
                    if diff_to > 0 {
                        left_overs.push((to - diff_to + 1, to));
                    }
                    if diff_from < 0 {
                        left_overs.push((from + diff_from, from - 1));
                    }

                    lefto_idx = lefto_idx + 1;
                    // println!("converted: {:?}", converted);
                    // println!("left overs: {:?}", left_overs);
                }
            }

            // println!("created left overs: {:?}", left_overs);
            // println!("created converted: {:?}", converted);

            if converted.len() >= 1 { ranges[range_idx] = converted[0]; } // first left is always the current range, maybe modified
            if converted.len() > 1 { ranges.extend(&converted[1..]); } // add newly found sub ranges
            if left_overs.len() > 1 { ranges.extend(&left_overs[1..]); } // add left overs which could not be converted
            range_idx = range_idx + 1;
        }

        println!("");
    }

    Ok(ranges)
}

struct Range {
    from: i64,
    to: i64,
}

// maps range to to mapping b with overflow on both ends returned as left overs
// return value (Mapped, (Lower Leftover, Upper Leftover))
//
// Note: for the dest of the mapping, a small correction of -1 is applied
// if 52 50 48, then the dst range 52 expects 48 values which counts up to 99, NOT 100
//
// Note: the left over ranges can include the same numbers as the mapped range
// 46 108 in 52 50 48 results in
// mapped: 52 99, lower: 46 49, higher: 98* 108
// *number 98 could not match and was kept while number 97 was mapped to 99
fn map_range(a: Range, b: Mapping) -> (Option<Range>, (Option<Range>, Option<Range>)) {
    if a.from <= b.source && a.to <= b.source {
        return (None, (Some(a), None)); // will keep the given range
    }
    if a.from >= (b.source + b.range - 1) && a.to >= (b.source + b.range - 1) {
        return (None, (None, Some(a)));
    }

    //       a.from .. a.to
    // b.src .............. b.src+range
    // diff from will be positive, diff to will be negative 
    let (diff_from, diff_to) = (a.from - b.source, a.to - (b.source + b.range - 1));
    let mapped = Range{
        from: b.dest + cmp::max(diff_from, 0),
        to: (b.dest + b.range - 1) + cmp::min(diff_to, 0)
    };

    // determine upper bound / lower bound sub ranges
    let lower_left_over = if diff_from < 0 { Some(Range{from: a.from, to: a.from + -diff_from - 1}) } else { None };
    let upper_left_over = if diff_to > 0 { Some(Range{from: a.to - diff_to + 1, to: a.to}) } else { None }; 

    (Some(mapped), (lower_left_over, upper_left_over))
}

fn test_map_ranges() {
    let mapped_1 = map_range(Range { from: 79, to: 92 }, Mapping{dest: 52, source: 50, range: 48});
    assert!(mapped_1.0.is_some()); assert!(mapped_1.1.0.is_none()); assert!(mapped_1.1.1.is_none());
    let main_1 = mapped_1.0.unwrap(); assert_eq!(main_1.from, 81); assert_eq!(main_1.to, 94);

    let mapped_2 = map_range(Range { from: 46, to: 92 }, Mapping{dest: 52, source: 50, range: 48});
    assert!(mapped_2.0.is_some()); assert!(mapped_2.1.0.is_some()); assert!(mapped_2.1.1.is_none());
    let main_2 = mapped_2.0.unwrap(); assert_eq!(main_2.from, 52); assert_eq!(main_2.to, 94);
    let lower_2 = mapped_2.1.0.unwrap(); assert_eq!(lower_2.from, 46); assert_eq!(lower_2.to, 49);

    let mapped_3 = map_range(Range { from: 46, to: 108 }, Mapping{dest: 52, source: 50, range: 48});
    assert!(mapped_3.0.is_some()); assert!(mapped_3.1.0.is_some()); assert!(mapped_3.1.1.is_some());
    let main_3 = mapped_3.0.unwrap(); assert_eq!(main_3.from, 52); assert_eq!(main_3.to, 99); 
    let lower_3 = mapped_3.1.0.unwrap(); assert_eq!(lower_3.from, 46); assert_eq!(lower_3.to, 49);
    let higher_3 = mapped_3.1.1.unwrap(); assert_eq!(higher_3.from, 98); assert_eq!(higher_3.to, 108);
}
