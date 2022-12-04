use std::{borrow::Borrow, ops::Range};

use reqwest::Error;

use crate::read_input::read_input;
fn section_in_range(range: Range<u32>, section: Vec<u32>) -> bool {
    if range.contains(&section[0]) && range.contains(&section[1]) {
        return true;
    }
    false
}
pub async fn day_four() -> Result<(), Error> {
    let res = read_input("https://adventofcode.com/2022/day/4/input").await?;
    // let zones: Vec<u8> = "5-13".split("-").map(|i| i.parse::<u8>()).collect();
    let contains: u32 = res
        .split('\n')
        .map(|section_assignment| {
            let sections: Vec<Vec<u32>> = section_assignment
                .split(",")
                .map(|range| {
                    range
                        .split("-")
                        .map(|zone| match zone.parse::<u32>() {
                            Ok(zone) => zone,
                            Err(err) => {
                                println!("{}", err);
                                0
                            }
                        })
                        .collect()
                })
                .collect();
            let mut sum_overlapping: u32 = 0;
            if sections.len() > 1 {
                let first_section = sections[0].clone();
                let second_section = sections[1].clone();
                let range1 = Range {
                    start: first_section[0],
                    end: first_section[1] + 1,
                };
                let range2 = Range {
                    start: second_section[0],
                    end: second_section[1] + 1,
                };
                if section_in_range(range1, second_section)
                    || section_in_range(range2, first_section)
                {
                    sum_overlapping += 1;
                }
            }
            sum_overlapping
        })
        .sum();

    println!("Day 4: {}", contains);
    Ok(())
}
