use crate::read_input::read_input;
use reqwest::Error;
const ASCII_BASE: u32 = 64;

pub async fn day_three() -> Result<(), Error> {
    println!("Day 3:");
    let res = read_input("https://adventofcode.com/2022/day/3/input").await?;
    let bags = res.split("\n");
    // part 1
    let error_sum: u32 = bags
        .clone()
        .map(|bag| {
            let middle = bag.len() / 2;
            let compartments = bag.split_at(middle);

            let mut error: u32 = 0;
            compartments.0.chars().for_each(|item| {
                if compartments.1.contains(item) {
                    if item.is_lowercase() {
                        error = item as u32 - 96
                    } else {
                        error = item as u32 - 38
                    };
                }
            });
            error
        })
        .sum();

    let bags_array: Vec<&str> = bags.collect();

    let mut badge: u32 = 0;
    let mut group = vec![];
    for bag in bags_array.iter() {
        if group.len() < 3 {
            group.push(bag)
        }
        if group.len() == 3 {
            let mut badge_char = '\0';
            bag.chars().for_each(|item| {
                if group.iter().all(|member| member.contains(item)) {
                    badge_char = item;
                } else {
                }
            });
            if badge_char.is_alphabetic() {
                if badge_char.is_lowercase() {
                    badge += badge_char as u32 - 96
                } else {
                    badge += badge_char as u32 - 38
                };
            }
            group = vec![];
        }
    }
    println!("Part 1: {}", error_sum);
    println!(" Part2: {}", badge);

    Ok(())
}
