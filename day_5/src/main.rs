use std::{time::Instant, vec};

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
    
    // get last numeric character from last line
    let last_line = container_structure.lines().last().unwrap().chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
    let num_stacks = last_line.last().unwrap().to_string().parse::<i32>().unwrap();
    println!("Number of stacks == {num_stacks}");
    for line in container_structure.lines() {
        let last_char = line.chars().enumerate().last().unwrap();
        let row = line.chars().enumerate().filter_map(|(index, c)| {
            // index == 0 && c.is_ascii_whitespace()) || c.is_ascii_alphabetic()  || (index == last_char.0 && last_char.1.is_ascii_whitespace()
         if index%4 == 1 && !c.is_numeric(){
            // println!("index = {index}");
            Some(c)
         } else {
            None
         }
        }).collect::<Vec<char>>();
        // println!("{row:?}");
        // println!("line length = {}",line.len());
        if container_stacks.is_empty(){
            for _index in 0..num_stacks {
                container_stacks.push(vec![])
            }
        }
        // print_stack(&container_stacks);
        let eval = row.iter().filter(|&a| !a.is_ascii_whitespace()).collect::<Vec<&char>>().is_empty();
        if !eval {
            for (stack, c) in row.iter().enumerate(){
                if c.is_ascii_alphabetic(){
                    container_stacks[stack].push(*c)
                }
            }
        }
        // let last_char = line.chars().last();
        // for (stack_id, char) in line {
            // if container_stacks.is_empty() || container_stacks.len() <= stack_id {
            //     container_stacks.push(vec![])
            // }
            // if location == 0 && char.is_ascii_whitespace() {
            //     row.push(' ');
            // } else if char.is_ascii_alphabetic() {
            //     row.push(char)
            // }
            // if char.is_alphabetic() {
            //     if container_stacks.len() <= stack_id {
            //         container_stacks.push(vec![])
            //     } else {
            //         container_stacks[stack_id].push(char)
            //     }
            // }
        // }
        // if last_char.unwrap().is_ascii_whitespace() {
        //     row.push(' ');
        // }
        // evaluate row to see if the whole thing is empty chars
        // let eval = row.iter().filter(|&a| !a.is_ascii_whitespace()).collect::<Vec<&char>>();
        // if !eval.is_empty(){
        //     container_stacks.push(row);
        // }

    }

    for stacks in &mut container_stacks {
        stacks.reverse()
    }


    // println!("Containers = ");
    // print_stack(&container_stacks);

    // create the commands
    let commands = commands
    .lines()
    .map(|a| a
        .split_whitespace()
        .filter_map(|a|a.parse::<i32>().ok())
            // .chars())
        // .filter_map(|a| 
        //         match a.to_string().parse::<i32>() {
        //             Ok(i) => Some(i),
        //             Err(_) => {panic!("invalid num {a}")},
        //         }
        // )
        .collect::<Vec<i32>>()
    ).map(|a| OperatorCommand::from_vec(&a))
    .collect::<Vec<OperatorCommand>>();
        
        
    for command in commands {
        command.execute(&mut container_stacks, &args);
        
    }

    // print_stack(&container_stacks);
    

    println!("Result = {}",get_top_containers_per_stack(&container_stacks));


    println!("executed in {}mc", now.elapsed().as_micros());

}

#[derive(Debug)]
pub struct OperatorCommand {
    move_amount: i32,
    from_stack: i32,
    to_stack: i32,
}

impl OperatorCommand {
    // fn new(container_amount: i32, from_stack: i32, to_stack: i32) -> Self {
    //     Self {
    //         move_amount: container_amount,
    //         from_stack,
    //         to_stack,
    //     }
    // }

    pub fn from_vec(command: &Vec<i32>) -> Self {
        Self { move_amount: command[0], from_stack: command[1], to_stack: command[2] }
    }

    fn execute(&self, container_stacks: &mut Vec<Vec<char>> , args: &AocArgs) {
        
        // println!("{self:?}");
        let from_stack_id = {self.from_stack - 1} as usize;
        let to_stack_id = {self.to_stack - 1} as usize;
       
        
        if args.part_one {
            let mut move_count = 0;
                // while we have `move_amount` left should keep moving

            while move_count < self.move_amount {
                

                let container_to_move = match container_stacks[from_stack_id].pop(){
                    Some(a) => a,
                    None => {
                        panic!("shouldn't get NONE on the pop")
                    },
                };

                
                container_stacks[to_stack_id].push(container_to_move);


        
                move_count +=1;

            }
        }

        if args.part_two {
            let stack_len = container_stacks[from_stack_id].len();
            let move_amount = self.move_amount as usize;
            let mut grabber = container_stacks[from_stack_id].split_off(stack_len - move_amount);

            container_stacks[to_stack_id].append(&mut grabber)
        }


    }
}

fn print_stack(container_stack: &Vec<Vec<char>>) {
    for (index, row) in container_stack.iter().enumerate() {
        println!("{index}: {row:?}")
    }
}

fn get_top_containers_per_stack(container_stack: &Vec<Vec<char>>) -> String {
    let last: Vec<char> = container_stack.iter().filter_map(|a| a.iter().last()).cloned().collect();
    last.iter().collect()
}
