use crate::read_input::read_input;
use reqwest::Error;
const ASCII_BASE: u32 = 64;

pub async fn day_three() -> Result<(), Error> {
    println!("Day 3:");
    let res = read_input("https://adventofcode.com/2022/day/3/input").await?;
    let bags = res.split("\n");
    // println!("{:?}", bags[0]);

    let error_sum: u32 = bags
        .map(|bag| {
            let middle = bag.len() / 2;
            let compartments = bag.split_at(middle);

            let mut error: u32 = 0;
            compartments.0.chars().for_each(|item| {
                if compartments.1.contains(item) {
                    if item.is_lowercase() {
                        error = item as u32 - 96
                    } else {
                        error = item as u32 - 64 + 26
                    };

                    println!("{}: {} ", item, error);
                }
            });
            error
        })
        .sum();
    println!("{}", error_sum);

    Ok(())
}
