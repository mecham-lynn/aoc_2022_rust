use std::{time::Instant, collections::HashMap};

//Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
fn main() {
    //Lowercase item types a through z have priorities 1 through 26.
    //Uppercase item types A through Z have priorities 27 through 52.
    let now = Instant::now();
    let input_txt = include_str!("../input-demo.txt");

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

    // a compartment in a bag is the string divided in half
    let bags_with_compartments: Vec<(Vec<char>, Vec<char>)> = bags.iter().map(|&a| {
            let (a, b) = a.split_at((a.len() - 1) / 2);
            // let zipped = a.chars().zip(b.chars()).map(|(a,b)|)
            (
                a.chars().collect(),
                b.chars().collect()
            )
        }
    ).collect();

    //Need to find matching items in each compartment
    let mut matches: Vec<char> = vec![];
    for (cmpt_1, cmpt_2) in bags_with_compartments {
        if let Some(res) = cmpt_1.iter().find(|&a| cmpt_2.binary_search(a).is_ok()) {
            matches.push(*res)
        }
    }

    let result_sum = matches.iter().fold(0, ||)

    todo!()
}
