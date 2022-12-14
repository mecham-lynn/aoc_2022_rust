use std::{time::Instant};

use helpers::AocArgs;
fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };

    // get the tree heights into a vec
    

    let mut visible_trees = 0;
    let mut scenic_scores = vec![];
    let trees = input_txt.lines().map(|a| a.chars().map(|a| a.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    let row_max = trees.len();
    let col_max = trees[0].len();

    // Check up down left and right (if possible) and iterate visible trees when the tree < all trees in that direction to the edge
    // This means a tree is visible if every tree is shorter than it in one direction  to the edge.
    for (row_id, col) in trees.iter().enumerate() {
    
        if row_id == 0 || row_id == trees.len() - 1 {
                visible_trees += col.len()
        } else {

            for (col_id, tree) in col.iter().enumerate() {
                let mut to_right = vec![];
                let mut to_left = vec![];
                let mut to_down = vec![];
                let mut to_up = vec![];
                
                if col_id == 0 || col_id == col_max - 1 {
                    visible_trees += 1;

                } else {
                    //check right
                    if col_id < col_max {
                        for i in col_id + 1..col_max {
                            to_right.push(trees[row_id][i])
                        }
                    }

                    //check left
                    if col_id > 0 {
                        for i in 0..col_id {
                            to_left.push(trees[row_id][i])
                        }
                        to_left.reverse()
                    }

                    // check down
                    if row_id > 0 {
                        for i in row_id + 1..row_max {
                            to_down.push(trees[i][col_id])
                        }
                    }

                    // check up
                    if row_id < row_max {
                        for i in (0..row_id).rev() {
                            to_up.push(trees[i][col_id]);
                        }
                    }
                    
                    if is_visible(tree, &to_right) || is_visible(tree, &to_left) || is_visible(tree, &to_down) || is_visible(tree, &to_up) {
                        visible_trees += 1
                    }

                    // let up_scenic_score = scenic_score(tree, &to_up);
                    // let down_scenic_score = scenic_score(tree, &to_down);
                    // let right_scenic_score = scenic_score(tree, &to_right);
                    // let left_scenic_score = scenic_score(tree, &to_left);

                    // println!("to_up: {:?}", &to_up);
                    // println!("tree {}, up: {}, down {}, left {}, right {}", &tree, up_scenic_score, down_scenic_score, left_scenic_score, right_scenic_score);
                    // println!("total_scenic score {}", up_scenic_score * down_scenic_score * left_scenic_score * right_scenic_score);

                    scenic_scores.push(scenic_score(tree, &to_right) * scenic_score(tree, &to_left) * scenic_score(tree, &to_down) * scenic_score(tree, &to_up));
                }
            }
        }
    }
    
    println!("Number of visible trees {}", visible_trees);
    // println!("scenic scores {:?}",&scenic_scores );
    println!("Max scenic score {:?}", scenic_scores.iter().max());

    println!("executed in {}mc", now.elapsed().as_micros());
}

fn is_visible(tree: &u32, slice: &[u32] ) -> bool {
    // if slice.is_empty(){
    //     return false
    // }
    // println!("tree evaluating: {}, against trees {:?}", tree, slice);
    let result = slice.iter().all(|a| tree > a);
    // println!("evaluated to {}", result);
    result
}

fn scenic_score(tree: &u32, slice: &[u32]) -> usize {
    let mut count = 0;
    for o_tree in slice {
        if o_tree < tree {
            count += 1
        } else  if o_tree >= tree {
            count += 1;
            break
        } else {
            break
        }
    }
    count
}