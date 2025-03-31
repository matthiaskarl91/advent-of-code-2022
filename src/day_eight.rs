use std::fs;

use reqwest::Error;

#[derive(Debug)]
pub struct Tree {
    x: usize,
    y: usize,
    height: i8,
}

impl Tree {
    fn new(x: usize, y: usize, height: i8) -> Self {
        Self { x, y, height }
    }
}

#[derive(Default, Debug)]
pub struct Forest {
    data: Vec<Tree>,
}

fn visible(forest: &Forest, tree: &Tree) -> bool {
    // let mut is_visible = false;
    let left_free = forest
        .data
        .iter()
        .filter(|t| t.x == tree.x && t.y < tree.y)
        .all(|t| t.height < tree.height);

    let right_free = forest
        .data
        .iter()
        .filter(|t| t.x == tree.x && t.y > tree.y)
        .all(|t| t.height < tree.height);

    let top_free = forest
        .data
        .iter()
        .filter(|t| t.x < tree.x && t.y == tree.y)
        .all(|t| t.height < tree.height);

    let bottom_free = forest
        .data
        .iter()
        .filter(|t| t.x > tree.x && t.y == tree.y)
        .all(|t| t.height < tree.height);

    return left_free || right_free || top_free || bottom_free;
}

fn scenic_score(forest: &Forest, tree: &Tree) -> u32 {
    let left_row: Vec<&Tree> = forest
        .data
        .iter()
        .filter(|t| t.x == tree.x && t.y < tree.y)
        .collect();

    let mut left_free = 0;
    if left_row.len() > 0 {
        left_free = left_row
            .iter()
            .position(|t| t.height >= tree.height)
            .unwrap_or(left_row.len()) as u32
            + 1;
    }

    let right_row: Vec<&Tree> = forest
        .data
        .iter()
        .filter(|t| t.x == tree.x && t.y > tree.y)
        .collect();

    let mut right_free = 0;
    if right_row.len() > 0 {
        right_free = right_row
            .iter()
            .position(|t| t.height >= tree.height)
            .unwrap_or(right_row.len()) as u32
            + 1;
    }

    let top_row: Vec<&Tree> = forest
        .data
        .iter()
        .filter(|t| t.x < tree.x && t.y == tree.y)
        .collect();

    let mut top_free = 0;
    if top_row.len() > 0 {
        top_free = top_row
            .iter()
            .position(|t| t.height >= tree.height)
            .unwrap_or(top_row.len()) as u32
            + 1;
    }

    let bottom_row: Vec<&Tree> = forest
        .data
        .iter()
        .filter(|t| t.x > tree.x && t.y == tree.y)
        .collect();

    let mut bottom_free = 0;
    if bottom_row.len() > 0 {
        bottom_free = bottom_row
            .iter()
            .position(|t| t.height >= tree.height)
            .unwrap_or(bottom_row.len()) as u32
            + 1;
    }

    println!(
        "l:{}, r:{}, t:{}, b:{}",
        left_free, right_free, top_free, bottom_free
    );
    return left_free * right_free * top_free * bottom_free;
}

pub fn get_forest(input: String) -> Forest {
    let forest_vec: Vec<Vec<i8>> = input
        .split("\n")
        .map(|row| {
            row.split("")
                .map(|tree| tree.parse::<i8>().unwrap_or(-1))
                .filter(|t| t >= &0)
                .collect()
        })
        .collect();

    let mut forest = Forest::default();
    let mut row_idx = 0;
    let mut result = 0;
    for row in &forest_vec {
        let mut col_idx = 0;
        for &height in row {
            let tree = Tree::new(col_idx, row_idx, height);

            forest.data.push(tree);
            col_idx += 1;
        }
        row_idx += 1;
    }

    forest
}

pub async fn execute() -> Result<(), Error> {
    // let mut res = read_input("https://adventofcode.com/2022/day/8/input").await?;
    let mut res = fs::read_to_string("day_eight.txt").unwrap();
    res.truncate(res.len() - 1);
    let forest_vec: Vec<Vec<i8>> = res
        .split("\n")
        .map(|row| {
            row.split("")
                .map(|tree| tree.parse::<i8>().unwrap_or(-1))
                .filter(|t| t >= &0)
                .collect()
        })
        .collect();

    let mut forest = Forest::default();
    let mut row_idx = 0;
    let mut result = 0;
    for row in &forest_vec {
        let mut col_idx = 0;
        for &height in row {
            let tree = Tree::new(col_idx, row_idx, height);

            forest.data.push(tree);
            col_idx += 1;
        }
        row_idx += 1;
    }

    forest.data.iter().for_each(|tree| {
        if visible(&forest, tree) {
            result += 1;
        }
    });

    let max_scenic_score: u32 = forest
        .data
        .iter()
        .map(|tree| scenic_score(&forest, tree))
        .max()
        .unwrap();

    println!("Day 8");

    println!("Part 1: {:?} max: {}", result, forest.data.len());
    println!("Part 2: {:?}", max_scenic_score);
    Ok(())
}

mod test {
    use super::*;
    #[test]
    fn ab() {
        let a = 0;
        let b = 3;
        assert!(a + b == 3, "doof");
    }

    #[test]
    fn part2() {
        let forest = get_forest(
            "30373
25512
65332
33549
35390"
                .to_string(),
        );
        assert_eq!(scenic_score(&forest, &forest.data[17]), 20);
        // println!("{:?}", forest);
    }
}
