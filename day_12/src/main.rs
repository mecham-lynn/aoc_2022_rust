use std::{time::Instant, collections::HashMap};

use helpers::AocArgs;

fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };

    let letters = "abcdefghijklmnop";
    let letter_map: HashMap<char, usize> = letters.chars().enumerate().map(|(num, c)| (c, num)).collect();

    let height_map: Vec<Vec<char>> = input_txt.lines().map(|a| a.chars().collect()).collect();

    for i in 0..height_map.len() {

    }

    

    println!("executed in {}mc", now.elapsed().as_micros());
}

