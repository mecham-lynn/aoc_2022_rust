
// const STR_DATA: &str = r"1000
// 2000
// 3000

// 4000

// 5000
// 6000

// 7000
// 8000
// 9000

// 10000";

use std::{cmp::Reverse, time::Instant};


// Each entry with 2 newlines is an elf
// PART 1 GOAL: Find the elf carrying the most calories. How many calories is that?
fn main() {

    let start = Instant::now();

    let input_text = include_str!("./input.txt");


    let elves = input_text.split("\n\n")
    .map(|s| s
        .split("\n")
        .map(|cal| cal.parse().unwrap())
        .collect::<Vec<i32>>())
    .collect::<Vec<Vec<i32>>>();

    let mut calories_per_elf = elves
    .iter()
    .map(|a| a.iter().sum())
    .collect::<Vec<i32>>();

    calories_per_elf.sort_by_key(|w| Reverse(*w));

    let largest: &i32 = calories_per_elf
    .iter()
    .max()
    .unwrap();

    let mut top_3 = 0;
    for (index, cal) in calories_per_elf.iter().enumerate() {
        if index < 3 {
            top_3 += cal
        } else {
            break;
        }
    }
    



    println!("Largest: {largest}");
    println!("Sum of Top 3: {top_3}");

    let duration = start.elapsed().as_micros();

    println!("Duration: {duration}mcs");
}
