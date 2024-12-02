use std::fs;

pub fn run(input_path: &String) -> u32 {
    let lines: Vec<String> = fs::read_to_string(input_path)
        .unwrap_or_default()
        .split('\n')
        .filter(|line| *line != "")
        .map(|line| String::from(line))
        .collect();
    assert_ne!(0, lines.len());

    // we know the length of each bit string
    let bit_string_length: usize = lines[0].len();

    // store the rows
    let mut report: Vec<Vec<u32>> = Vec::with_capacity(lines.len());
    for line in lines {
        assert_eq!(bit_string_length, line.len());

        report.push(line.chars()
            .map(|bit| String::from(bit).parse::<u32>().unwrap())
            .collect());
    }

    // calculate ratings & store indices
    let mut oxygen_idc: Vec<usize> = (1..report.len()).collect();
    let mut co2_idc: Vec<usize> = (1..report.len()).collect();

    for bit_idx in 0..bit_string_length {
        let oxy_threshold: u32 = f64::ceil(oxygen_idc.len() as f64 / 2f64) as u32; // upper half
        let co2_threshold: u32 = f64::ceil(co2_idc.len() as f64 / 2f64) as u32;

        let oxy_col_sum: u32 = oxygen_idc.iter()
            .map(|row| report[*row as usize][bit_idx]).sum();
        let co2_col_sum: u32 = co2_idc.iter()
            .map(|row| report[*row as usize][bit_idx]).sum();

        if oxygen_idc.len() > 1 {
            oxygen_idc = oxygen_idc
                .into_iter() // into_iter() uses values instead of references with iter()
                .filter(|row| report[*row][bit_idx] == (oxy_col_sum >= oxy_threshold) as u32)
                .collect();
        }
        if co2_idc.len() > 1 {
            co2_idc = co2_idc
                .into_iter() // into_iter() uses values instead of references with iter()
                .filter(|row| report[*row][bit_idx] == (co2_col_sum < co2_threshold) as u32)
                .collect();
        }
    }
    assert_eq!(oxygen_idc.len(), 1);
    assert_eq!(co2_idc.len(), 1);
    
    // lazy way to convert binary => decimal
    let oxygen_rating: u32 = (1..bit_string_length+1)
        .map(|i| u32::pow(2, (bit_string_length - i) as u32) * report[oxygen_idc[0]][i-1])
        .sum();
    let co2_scrubber_rating: u32 = (1..bit_string_length+1)
        .map(|i| u32::pow(2, (bit_string_length - i) as u32) * report[co2_idc[0]][i-1])
        .sum();

    oxygen_rating * co2_scrubber_rating
}
