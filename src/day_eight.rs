use std::fs;

use reqwest::Error;

use crate::read_input::read_input;

#[derive(Debug)]
struct Tree {
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
struct Forest {
    data: Vec<Tree>,
}

fn visible(forest: &Forest, tree: &Tree) -> bool {
    // let mut is_visible = false;
    let is_visible = forest
        .data
        .iter()
        // .filter(|t| t.x != tree.x && t.y != tree.y)
        .any(|t| {
            // println!("in Forest: {:?}, tree: {:?}", t, tree);
            if t.x == tree.x && t.y == tree.y && t.height == tree.height {
                return false;
            } else if t.x == tree.x && t.height < tree.height {
                // same row
                // if t.height < tree.height {
                // is_visible = true;

                return true;
                // }
            } else if t.y == tree.y && t.height < tree.height {
                // if t.height < tree.height {
                // is_visible = true;
                return true;
                // }
            } else {
                return false;
            }
        });
    // println!("{}", is_visible);
    return is_visible;
}

// fn is_visible(forest: &Vec<Vec<usize>>, tree: &Tree) -> bool {
//     let mut row_idx = 0;
//     for row in forest {
//         let mut col_idx = 0;
//         for col in row {
//             if tree.y == row_idx {
//                 if col < &tree.height {
//                     println!(
//                         "x: {}, y: {}, height: {} vs: {:?}",
//                         row_idx, col_idx, col, tree
//                     );
//                     return true;
//                     // } else {
//                     //     print
//                 }
//             }
//             col_idx += 1;
//         }
//         row_idx += 1;
//     }

//     return true;
// }

pub async fn execute() -> Result<(), Error> {
    // let res = read_input("https://adventofcode.com/2022/day/8/input").await?;
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
        if tree.x == 0 || tree.x == forest_vec.len() - 1 {
            // result += 0;
            // continue;
            return;
        }
        if tree.y == 0 || tree.y == forest_vec[0].len() - 1 {
            // result += 0;
            return;
        }
        println!("{:?}", tree);
        if visible(&forest, tree) {
            result += 1;
        }
    });

    println!("Day 8");

    println!("Part 1: {:?} max: {}", result, forest.data.len());
    println!("{:?}", res);
    // part two
    // println!("Part 2: {:?}", parsed.iter().take(3).sum::<u32>());
    Ok(())
}

// fn visible_from_left(row: Vec<usize>, tree: usize) -> bool {}
