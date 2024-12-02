use std::{fs::read_to_string, collections::HashMap};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::cmp;

const SEEDS: &str = "seeds: ";

#[derive(Debug, Clone, Copy)]
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
    let seeds: Vec<Range> = raw_seeds.iter().map(|n| Range{from: *n, to: *n}).collect();

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


    let mut min_locations_all: Vec<Range> = Vec::new();
    for seed in seeds.iter() {
        let ranges = find_locations(*seed, &almanac)?;
        let min = ranges.iter().min_by_key(|x| x.from).ok_or(std::fmt::Error)?;
        min_locations_all.push(*min);
    }
    println!("min locations all: {:?}", min_locations_all);
    let min_all = min_locations_all.iter().min_by_key(|x| x.from).ok_or(std::fmt::Error)?;

    println!();
    println!("======================================================================="); 
    // find locations for every tuple of seeds [(seed, range)], convert to [(from, to)]
    // https://stackoverflow.com/a/54420325
    let all_seeds_raw: Vec<Range> = raw_seeds
        .chunks(2).map(|seed| Range{from: seed[0], to: seed[0] + (seed[1] - 1)}).collect::<Vec<Range>>();

    let mut min_locations_tuple: Vec<Range> = Vec::new();
    for seed in all_seeds_raw.iter() {
        let ranges = find_locations(*seed, &almanac)?;
        
        let sum_diff_ranges = ranges.iter().map(|r| (r.from - r.to).abs()).sum::<i64>() + (ranges.len() as i64) - 1; 
        assert_eq!(sum_diff_ranges, (seed.from - seed.to).abs());

        let min = ranges.iter().min_by_key(|x| x.from).ok_or(std::fmt::Error)?;
        min_locations_tuple.push(*min);
    }
    println!("min locations tuple: {:?}", min_locations_tuple);
    let min_tuple = min_locations_tuple.iter().min_by_key(|x| x.from).ok_or(std::fmt::Error)?;

    println!();
    println!("=======================================================================");
    Ok((min_all.from, min_tuple.from))
}

fn find_locations(seed: Range, almanac: &HashMap<AlmanacType, Vec<Mapping>>) -> Result<Vec<Range>, Box<dyn std::error::Error>> {
    let mut ranges: Vec<Range> = vec![seed]; // will contain all ranges
    println!("seed: {:?}", seed);
 
    for key in AlmanacType::iter() {
        let mappings = almanac.get(&key).ok_or(std::fmt::Error)?;
        let mut mapped_ranges: Vec<Range> = Vec::new();
        // println!("{:?} | ranges {:?}", key, ranges);

        // work on all ranges
        for range in ranges.iter() {
            let mut left_overs = vec![range.clone()];
            let mut converted: Vec<Range> = vec![];

            // determine sub ranges
            // left overs are ranges to be evaluated
            for mapping in mappings.iter() {

                let lefto_idx: i32 = 0;
                while lefto_idx < left_overs.len() as i32 {
                    let r = left_overs[lefto_idx as usize];

                    let (o_mapped, o_leftovers) = map_range(r, *mapping);
                    if o_mapped.is_none() {
                        break
                    }

                    let mapped = o_mapped.unwrap();
                    converted.push(mapped);
                    left_overs.remove(lefto_idx as usize); // if converted, remove current

                    if let Some(lower) = o_leftovers.0 {
                        left_overs.push(lower)
                    }
                    if let Some(upper) = o_leftovers.1 {
                        left_overs.push(upper)
                    }
                    break // only map once per mapping
                }
            }

            // println!("mapped {:?}, left overs: {:?}", converted, left_overs);
            if converted.len() > 0 { mapped_ranges.extend(&converted); }
            if left_overs.len() > 0 { mapped_ranges.extend(&left_overs); } // add left overs which could not be converted
            assert!((converted.len() + left_overs.len()) > 0);
        }

        println!();
        ranges = mapped_ranges.clone(); // mapped are the next ones for the next step
    }
    println!("return {:?}", ranges);
    println!();

    Ok(ranges)
}

#[derive(Debug, Copy, Clone)]
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
    if a.from < b.source && a.to < b.source {
        return (None, (Some(a), None)); // will keep the given range
    }
    if a.from > (b.source + b.range - 1) && a.to > (b.source + b.range - 1) {
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
    // println!("{}, {}", diff_from, diff_to);
    assert!(mapped.from <= mapped.to);
    assert!(mapped.from >= 0); assert!(mapped.to >= 0);

    // determine upper bound / lower bound sub ranges
    let lower = if diff_from < 0 { Some(Range{from: a.from, to: a.from + -diff_from - 1}) } else { None };
    let upper = if diff_to > 0 { Some(Range{from: a.to - diff_to + 1, to: a.to}) } else { None }; 
    if let Some(l) = lower { assert!(l.from <= l.to); assert!(l.from >= 0); assert!(l.to >= 0); }
    if let Some(u) = upper { assert!(u.from <= u.to); assert!(u.from >= 0); assert!(u.to >= 0); }

    // sanity check, split ranges' difference add up the original difference of a.from - a.to
    let mut mapped_diff_sum: i64 = (mapped.from - mapped.to).abs();
    if let Some(l) = lower { mapped_diff_sum += (l.from - l.to).abs() + 1; } // one gets lost on splitting
    if let Some(u) = upper { mapped_diff_sum += (u.from - u.to).abs() + 1; } // one gets lost on splitting
    assert_eq!(mapped_diff_sum, (a.from - a.to).abs());

    (Some(mapped), (lower, upper))
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

    let mapped_4 = map_range(Range { from: 55, to: 200 }, Mapping{dest: 56, source: 200, range: 10});
    assert!(mapped_4.0.is_some()); assert!(mapped_4.1.0.is_some()); assert!(mapped_4.1.1.is_none());
    let main_4 = mapped_4.0.unwrap(); assert_eq!(main_4.from, 56); assert_eq!(main_4.to, 56);
    let lower_4 = mapped_4.1.0.unwrap(); assert_eq!(lower_4.from, 55); assert_eq!(lower_4.to, 199); 
}
