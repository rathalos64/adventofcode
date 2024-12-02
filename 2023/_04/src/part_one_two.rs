use std::fs::read_to_string;

pub fn run(input_file: &String) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let body = read_to_string(input_file)?;
    let lines: Vec<&str> = body.lines().collect();
    assert_ne!(0, lines.len());

    let matches_per_scratch = lines.iter().map(|line| {
        let card: Vec<&str> = line.split(":").collect();
        assert_eq!(card.len(), 2);

        let numbers: Vec<&str> = card[1].split("|").collect();
        assert_eq!(numbers.len(), 2);

        let winning: Vec<i32> = numbers[0]
            .split_whitespace()
            .map(|s| s.trim().parse::<i32>().map_err(|x| x.to_string()))
            .collect::<Result<Vec<i32>, String>>()?;
        let found: Vec<i32> = numbers[1]
            .split_whitespace()
            .map(|s| s.trim().parse::<i32>().map_err(|x| x.to_string()))
            .collect::<Result<Vec<i32>, String>>()?;
        Ok(found.into_iter().filter(|n| winning.contains(n)).collect::<Vec<i32>>().len() as i32)
    }).collect::<Result<Vec<i32>, String>>()?;

    // part_one
    let sum_scratchcards: i32 = matches_per_scratch.iter()
        .filter(|&&mps| mps > 0)
        .map(|&mps| i32::pow(2, mps as u32 - 1))
        .sum();
    
    // part_two
    let mut nscratchcards: i32 = 0;
    for (index, _) in matches_per_scratch.iter().enumerate() {
        traverse(&matches_per_scratch, &mut nscratchcards, index as i32);
    }

    Ok((sum_scratchcards, nscratchcards))
}

fn traverse(cards: &Vec<i32>, ncards: &mut i32, base_index: i32) {
    (*ncards) += 1;
    for i in 0..(cards[base_index as usize]) {
        traverse(cards, ncards, base_index + (i + 1)) // 10 with 2 resuls leads to 11, 12 as index
    }
}