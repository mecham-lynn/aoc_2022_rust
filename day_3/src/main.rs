use std::{time::Instant, collections::HashMap};

use argh::FromArgs;
use itertools::Itertools;

//Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
fn main() {
    //Lowercase item types a through z have priorities 1 through 26.
    //Uppercase item types A through Z have priorities 27 through 52.
    let now = Instant::now();
   
    let args: AocD3Args = argh::from_env();
    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };

    let char_str = "abcdefghijklmnopqrstuvwxyz";

    let mut priorities: HashMap<char, usize> = HashMap::new();

    // iterate over the char_str and populate the priorities map

    // char_str.chars()
    // .enumerate()
    // .map(|(priority, letter)| {
    //     priorities.insert(letter.clone(), priority + 1); 
    //     let uppercase = letter.to_ascii_uppercase();
    //     priorities.insert(uppercase, priority + 27);
    // }
    // );

    for (priority, letter) in char_str.chars().enumerate() {
        priorities.insert(letter.clone(), priority + 1);
        priorities.insert(letter.to_ascii_uppercase(), priority + 27);
    }

    // Now that we have the priority-mapping we need to break the input string up into bags
    let bags: Vec<&str> = input_txt.split("\n").collect();

    if args.part_one {
        // a compartment in a bag is the string divided in half
        let mut bags_with_compartments: Vec<(Vec<char>, Vec<char>)> = bags.iter().map(|&a| {
                let (a, b) = a.split_at((a.len()) / 2);
                // let zipped = a.chars().zip(b.chars()).map(|(a,b)|)
                (
                    a.chars().collect(),
                    b.chars().collect()
                )
            }
        ).collect();
        

        // println!("Bags: ");
        // for bag in &bags_with_compartments {
        //     println!("{:?}", bag)
        // }

        //Need to find matching items in each compartment
        let mut matches: Vec<char> = vec![];

        
        for (cmpt_1, cmpt_2) in bags_with_compartments.iter_mut() {
            cmpt_1.sort();
            cmpt_2.sort();

            if let Some(res) = cmpt_1.iter().find(|&a| cmpt_2.binary_search(a).is_ok()) {
                matches.push(*res)
            }
        }
        // println!("matches: {:?}", &matches);
        let result_sum: usize = matches.iter().map(|a| priorities.get(&a).unwrap()).sum();

        println!("SUM of all priorities: {result_sum}");
    }

    if args.part_two {
        // chunk the bags into groups of 3
        let groups = bags.chunks(3);
        
        // let mut priorities: Vec<usize> = vec![];
        let mut results = vec![];

        for group in groups {
            let mut match_counter: HashMap<char, usize> = HashMap::new();
            group.iter()
            .for_each(|&i| i
                .chars()
                .unique()
                .for_each(|c| *match_counter
                    .entry(c)
                    .or_insert(0) += 1
                )
            );
            // let (c, i) = match_counter.iter().filter(|(_item, count)| **count >= 3).reduce(f);
            let result = match_counter.iter().reduce(|a, b| if a.1 >= b.1 {a} else {b});
            match result {
                Some((item, _count)) => results.push(priorities.get(item).unwrap().clone()),
                None => panic!("we are guaranteed to have one matching item per bag somehow that isn't true"),
            }
        }

        println!("Sum of all priorities: {}", results.iter().sum::<usize>())
        
    }


    println!("Finished in {}mc", now.elapsed().as_micros());
}


#[derive(FromArgs)]
/// Set of args for Day 3
pub struct AocD3Args{
    /// enables part_one functionality
    #[argh(switch)]
    part_one: bool,

    /// enables part_two functionality
    #[argh(switch)]
    part_two: bool,

    /// flag on whether to use demo text or the actual text
    #[argh(switch)]
    demo_text: bool,
}