use reqwest::Error;
use std::{io::Read, str};

use crate::read_input::read_stream;

pub async fn execute() -> Result<(), Error> {
    let stream = read_stream("https://adventofcode.com/2022/day/6/input")
        .await?
        .bytes()
        .await?;
    let stream_part2 = read_stream("https://adventofcode.com/2022/day/6/input")
        .await?
        .bytes()
        .await?;

    let mut result = 0;
    for idx in 0..stream.len() - 3 {
        let window = stream.slice(idx..idx + 4);
        let mut window_vec: Vec<&u8> = window.iter().map(|element| element).collect();

        window_vec.sort();
        window_vec.dedup();
        if window_vec.len() == 4 {
            // println!("{}", idx + 4);
            result = idx + 4;
            break;
        }
    }

    let mut result_message = 0;
    for message_idx in 0..stream_part2.len() - 13 {
        let message_window = stream_part2.slice(message_idx..message_idx + 14);
        let mut message_window_vec: Vec<&u8> =
            message_window.iter().map(|element| element).collect();
        message_window_vec.sort();
        message_window_vec.dedup();
        if message_window_vec.len() == 14 {
            result_message = message_idx + 14;
            break;
        }
    }
    println!("Day 6: ");
    println!("Part 1: {}", result);
    println!("Part 2: {}", result_message);
    Ok(())
}
