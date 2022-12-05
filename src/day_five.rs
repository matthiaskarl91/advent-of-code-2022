use std::{any, str::FromStr};

use reqwest::Error;

use crate::read_input::read_input;

#[derive(Debug)]
struct Crane {
    stack: Vec<Vec<char>>,
}

impl Crane {}

impl FromStr for Crane {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Vec<char>> = Vec::new();
        for line in s.lines().rev() {
            let mut crane_idx = 0;
            let mut idx = 0;
            while idx < line.len() {
                if line[idx..].starts_with("[") {
                    let c = line.chars().nth(idx + 1).expect("it exists");
                    if stack.len() <= crane_idx {
                        stack.push(Vec::new());
                    }
                    stack[crane_idx].push(c);
                }
                idx += 4;
                crane_idx += 1;
            }
        }
        return Ok(Crane { stack: stack });
    }
}

pub async fn execute() -> Result<(), Error> {
    let res = read_input("https://adventofcode.com/2022/day/5/input").await?;
    let (crane, moves) = res.split_once("\n\n").expect("not ");
    let crane = crane.parse::<Crane>();

    // let stack = separated[0];
    // let crates: Vec<&str> = stack.split("\n").collect();
    println!("{:?}", crane);
    println!("{:?}", moves);
    Ok(())
}
