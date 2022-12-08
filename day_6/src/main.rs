use std::time::Instant;

use helpers::AocArgs;
use itertools::Itertools;

fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };

    
    let char_buffer = if args.part_one {
        4_usize
    } else {
        14_usize
    };

    for i in 0..input_txt.len() {
        let four_digit_slice = &input_txt[i..(i+char_buffer)];
        let slice = four_digit_slice.chars().unique().collect::<Vec<char>>();
        if slice.len() == char_buffer {
            println!("first marker char found at {}", i + char_buffer);
            break;
        }
    }


    println!("executed in {}mc", now.elapsed().as_micros());
}

