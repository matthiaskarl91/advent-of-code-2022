use reqwest::Error;

use crate::read_input::read_input;

pub async fn day_one() -> Result<(), Error> {
    let res = read_input("https://adventofcode.com/2022/day/1/input").await?;
    let lines = res.split("\n\n");
    let mut parsed: Vec<u32> = lines
        .map(|line| line.split("\n").flat_map(|num| num.parse::<u32>()).sum())
        .collect();

    parsed.sort_by(|a, b| b.cmp(a));
    println!("Day 1");
    println!("Part 1: {:?}", parsed[0]);

    // part two
    println!("Part 2: {:?}", parsed.iter().take(3).sum::<u32>());
    Ok(())
}
