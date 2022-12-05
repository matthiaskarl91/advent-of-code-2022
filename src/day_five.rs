use std::{any, str::FromStr};

use anyhow::anyhow;
use reqwest::Error;

use crate::read_input::read_input;

#[derive(Debug)]
struct Crane {
    stack: Vec<Vec<char>>,
}

impl Crane {
    fn move_crate_part_one(&mut self, move_instruction: &Move) {
        for _ in 0..move_instruction.move_crates {
            let from = self.stack[move_instruction.from]
                .pop()
                .expect("should work");
            self.stack[move_instruction.to].push(from);
        }
    }
    fn move_crate_part_two(&mut self, move_instruction: &Move) {
        let mut batch_from = Vec::new();
        for _ in 0..move_instruction.move_crates {
            // self.stack[move_instruction.from].reverse();
            let from = self.stack[move_instruction.from]
                .pop()
                .expect("should work");
            batch_from.push(from);
        }

        while batch_from.len() > 0 {
            self.stack[move_instruction.to].push(batch_from.pop().expect("should work"));
        }
        // batch_from.reverse();
        // for from_iter in batch_from {
        //     self.stack[move_instruction.to].push(from_iter);
        // }
    }
}

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

#[derive(Debug)]
struct Move {
    move_crates: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let move_line: Vec<&str> = s.split(" ").collect();
        if (move_line.len() <= 5) {
            return Err(anyhow!("Missing attribute"));
        }
        let move_crates = move_line[1].parse::<usize>().unwrap();
        let from = move_line[3].parse::<usize>().unwrap() - 1;
        let to = move_line[5].parse::<usize>().unwrap() - 1;
        Ok(Move {
            move_crates: move_crates,
            from: from,
            to: to,
        })
    }
}

pub async fn execute() -> Result<(), Error> {
    let res = read_input("https://adventofcode.com/2022/day/5/input").await?;
    let (crane, moves) = res.split_once("\n\n").expect("not ");
    let mut crane = crane.parse::<Crane>().unwrap();
    let moves: Vec<Move> = moves
        .split("\n")
        .map(|move_str| match move_str.parse::<Move>() {
            Ok(mov) => mov,
            Err(err) => Move {
                move_crates: 0,
                from: 0,
                to: 0,
            },
        })
        .collect();
    // let stack = separated[0];
    // let crates: Vec<&str> = stack.split("\n").collect();
    // println!("{:?}", crane);
    // println!("{:?}", moves);

    for move_instruction in moves {
        crane.move_crate_part_two(&move_instruction);
        // let destination_crates = &crane.stack[from];

        // let target_crates = &crane.stack[move_instruction.to];
    }

    for stack in crane.stack {
        print!("{:?}", stack.last().unwrap());
    }

    Ok(())
}
