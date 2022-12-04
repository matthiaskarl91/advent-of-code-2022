use std::ops::Range;

use reqwest::Error;

use crate::read_input::read_input;

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
            println!("{:?}", sections);
            0
        })
        .sum();

    Ok(())
}
