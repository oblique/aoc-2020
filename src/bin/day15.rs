use anyhow::Result;
use std::collections::HashMap;

fn parse() -> Result<Vec<u32>> {
    std::fs::read_to_string("./data/input15.txt")?
        .trim()
        .split(',')
        .map(|n| n.parse().map_err(Into::into))
        .collect()
}

fn start_game(last_turn: u32) -> Result<u32> {
    let input = parse()?;

    let mut prev_num = input[input.len() - 1];
    let mut spoken_nums: HashMap<_, _> =
        input.into_iter().enumerate().map(|(i, k)| (k, i as u32 + 1)).collect();
    let start_turn = (spoken_nums.len() + 1) as u32;

    for turn in start_turn..=last_turn {
        let num = if !spoken_nums.contains_key(&prev_num) {
            0
        } else {
            turn - 1 - spoken_nums[&prev_num]
        };

        spoken_nums.insert(prev_num, turn - 1);
        prev_num = num;
    }

    Ok(prev_num)
}

fn main() -> Result<()> {
    println!("part1: {}", start_game(2020)?);
    println!("part2: {}", start_game(30000000)?);
    Ok(())
}
