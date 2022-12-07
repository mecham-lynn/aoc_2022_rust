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

    
    for line in container_structure.lines() {
        let mut row = vec![];
        let last_char = line.chars().last();
        for (location, char) in line.chars().enumerate() {
            if location == 0 && char.is_ascii_whitespace() {
                row.push(' ');
            } else if char.is_ascii_alphabetic() {
                row.push(char)
            }
        }
        if last_char.unwrap().is_ascii_whitespace() {
            row.push(' ');
        }
        // evaluate row to see if the whole thing is empty chars
        let eval = row.iter().filter(|&a| !a.is_ascii_whitespace()).collect::<Vec<&char>>();
        if !eval.is_empty(){
            container_stacks.push(row);
        }

    }


    println!("Containers = {:?}",container_stacks);

    // create the commands
        let command = commands
        .lines()
        .map(|a| a
            .split_whitespace()
            .flat_map(|a|a
                .chars())
            .filter_map(|a| 
                    a.to_string().parse::<i32>().ok()
            )
            .collect::<Vec<i32>>()
        ).map(|a| )
        
        
        println!("{command:?}");

    println!("executed in {}mc", now.elapsed().as_micros());

}

pub struct OperatorCommand {
    move_amount: i32,
    from_stack: i32,
    to_stack: i32,
}

impl OperatorCommand {
    fn new(container_amount: i32, from_stack: i32, to_stack: i32) -> Self {
        Self {
            move_amount: container_amount,
            from_stack,
            to_stack,
        }
    }

    fn execute(&self, container_stacks: &mut Vec<Vec<char>>) {
        
        let from_stack_id = {self.from_stack - 1} as usize;
        let to_stack_id = {self.to_stack - 1} as usize;
        // while we have `move_amount` left should keep moving
        let mut move_count = 0;
        while move_count < self.move_amount {
            let new_container_stack = container_stacks.clone();
            // grab the container from it's existing stack
            let (stack_depth, container) = {
                new_container_stack[from_stack_id].iter().enumerate().find(|(_s, &container)| container.is_alphabetic()).unwrap()
               
            };
            let new_stack_depth = container_stacks[to_stack_id].iter().enumerate().find(|(_s, &container)| container.is_alphabetic()).unwrap().0 -1;

            container_stacks[from_stack_id][stack_depth] = ' ';
            container_stacks[to_stack_id][new_stack_depth] = container.clone();
            move_count +=1;
        }


    }
}

fn print_stack(container_stack: &Vec<Vec<char>>) {
    for row in container_stack {
        println!("{row:?}")
    }
}
