use std::time::Instant;

use helpers::AocArgs;

fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };

    // separate the input
    let (container_structure, commands) = input_txt.split_once("\n\n").unwrap();
    
    let mut container_stacks: Vec<Vec<char>> = vec![];


    let mut add_next: bool = false;
    
    for ( stack, line) in container_structure.lines().enumerate() {
        let mut location_on_stack = 0_usize;
        for (location, char) in line.chars().enumerate() {
            if location == 0 && char.is_ascii_whitespace() {
                container_stacks[stack][location_on_stack] = ' ';
                location_on_stack += 1;
            } else if char.is_ascii_alphabetic() {
                container_stacks[stack][location_on_stack] = char
            }
        }
    }
    //container structure parsing
    // let containers:Vec<Vec<char>> = container_structure.split("\n")
    //     .inspect(|a| println!("current container string {a}"))
    //     .map(|a| a
    //         .split_terminator(&['[',']'])
    //         .collect::<Vec<&str>>()
    //         .iter()
    //         .flat_map(|c| c.chars())
    //         .collect()
    //     )
    //     .collect();

    println!("Containers = {:?}",container_stacks);


    println!("executed in {}mc", now.elapsed().as_micros());

}

pub struct OperatorCommand {
    move_amount: i32,
    from_stack: i32,
    to_stack: i32,
}
