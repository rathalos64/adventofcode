use std::fs;

pub fn run(input_path: &String) -> u32 {
    let lines: Vec<String> = fs::read_to_string(input_path)
        .unwrap_or_default()
        .split('\n')
        .filter(|line| *line != "")
        .map(|line| String::from(line))
        .collect();
    assert_ne!(0, lines.len());

    // given the binary nature of the diagnostic report
    // we already know threshold to determine a majority (gamma) of minority (epsilon)
    // of 1
    // furthermore, we know the length of each bit string
    let threshold: u32 = lines.len() as u32 / 2;
    let bit_string_length: usize = lines[0].len();

    // store the columns
    let mut report: Vec<Vec<u32>> = (0..bit_string_length)
        .map(|_| Vec::new())
        .collect();

    for line in lines {
        assert_eq!(bit_string_length, line.len());

        let bits: Vec<u32> = line.chars()
            .map(|bit| String::from(bit).parse::<u32>().unwrap())
            .collect();
        for i in 0..bit_string_length {
            report[i].push(bits[i]);
        }
    }

    // generate the decimal metrics from a binary representation
    let gamma: u32 = (1..bit_string_length+1)
        .map(|i| (
            u32::pow(2, bit_string_length as u32 - i as u32), // binary decimal (e.g. 2, 4, 8)
            report[i-1].iter().sum::<u32>()
        ))
        .map(|(binary_dec, bits_sum)| (bits_sum > threshold) as u32 * binary_dec)
        .sum();
    let epsilon: u32 = (1..bit_string_length+1)
        .map(|i| (
            u32::pow(2, bit_string_length as u32 - i as u32), // binary decimal (e.g. 16, 32, 64)
            report[i-1].iter().sum::<u32>()
        ))
        .map(|(binary_dec, bits_sum)| (bits_sum < threshold) as u32 * binary_dec)
        .sum();

    gamma * epsilon
}
