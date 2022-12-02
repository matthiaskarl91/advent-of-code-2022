use std::str::FromStr;

use crate::read_input::read_input;
use reqwest::Error;

enum Enemy {
    A, // Rock
    B, // Paper
    C, // Scissor
}

impl FromStr for Enemy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Enemy::A),
            "B" => Ok(Enemy::B),
            "C" => Ok(Enemy::C),
            _ => Err(()),
        }
    }
}
enum Player {
    X, // Rock
    Y, // Paper
    Z, // Scissor
}

impl FromStr for Player {
    type Err = ();

    fn from_str(input: &str) -> Result<Player, Self::Err> {
        match input {
            "X" => Ok(Player::X),
            "Y" => Ok(Player::Y),
            "Z" => Ok(Player::Z),
            _ => Err(()),
        }
    }
}

fn get_score_part_1(take1: Player, take2: Enemy) -> u32 {
    match take1 {
        Player::X => {
            let outcome_score = match take2 {
                Enemy::A => 3,
                Enemy::B => 0,
                Enemy::C => 6,
            };
            outcome_score + 1
        }
        Player::Y => {
            let outcome_score = match take2 {
                Enemy::A => 6,
                Enemy::B => 3,
                Enemy::C => 0,
            };
            outcome_score + 2
        }
        Player::Z => {
            let outcome_score = match take2 {
                Enemy::A => 0,
                Enemy::B => 6,
                Enemy::C => 3,
            };
            outcome_score + 3
        }
    }
}

fn get_score_part_2(take1: Player, take2: Enemy) -> u32 {
    match take1 {
        Player::X => {
            let outcome_score = match take2 {
                Enemy::A => 0 + 3,
                Enemy::B => 0 + 1,
                Enemy::C => 0 + 2,
            };
            outcome_score
        }
        Player::Y => {
            let outcome_score = match take2 {
                Enemy::A => 3 + 1,
                Enemy::B => 3 + 2,
                Enemy::C => 3 + 3,
            };
            outcome_score
        }
        Player::Z => {
            let outcome_score = match take2 {
                Enemy::A => 6 + 2,
                Enemy::B => 6 + 3,
                Enemy::C => 6 + 1,
            };
            outcome_score
        }
    }
}

pub async fn day_two() -> Result<(), Error> {
    let res = read_input("https://adventofcode.com/2022/day/2/input").await;
    println!("Day 2");
    if let Ok(res) = res {
        let encounters: Vec<Vec<&str>> = res
            .split("\n")
            .map(|line| {
                let game: Vec<&str> = line.split(" ").map(|team| team).collect();

                game
            })
            .collect();
        let score_part_1: u32 = encounters
            .iter()
            .map(|encounter| {
                if encounter.len() < 2 {
                    return 0;
                }
                let player: Player = Player::from_str(encounter[1]).unwrap();
                let enemy: Enemy = Enemy::from_str(encounter[0]).unwrap();
                get_score_part_1(player, enemy)
            })
            .sum();
        println!("Part 1: Score: {}", score_part_1);
        let score_part_2: u32 = encounters
            .iter()
            .map(|encounter| {
                if encounter.len() < 2 {
                    return 0;
                }
                let player: Player = Player::from_str(encounter[1]).unwrap();
                let enemy: Enemy = Enemy::from_str(encounter[0]).unwrap();
                get_score_part_2(player, enemy)
            })
            .sum();
        println!("Part 2: Score: {}", score_part_2);
    }

    Ok(())
}
