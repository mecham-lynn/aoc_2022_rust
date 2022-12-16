use std::{time::Instant, str::FromStr, collections::HashMap, ops::Add, fmt::Display};

use helpers::AocArgs;

fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let mut x_vals: HashMap<usize, i32> = HashMap::new();
    let mut current_x_val = 1;

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };

    let instructions: Vec<Instructions> = input_txt.lines().map(|f| Instructions::from_str(f).unwrap()).collect();


    let mut screen = Screen::new();

    let mut sprite = Sprite::new();
    

    // let mut previous_instruction: Option<i32> = None;
    let mut cycle_count = 1_usize;

    for instruction in instructions.iter() {
        // loop through each instruction and
        // check execution_buffer[0].0 == cycle_count
        //      -- increment current_x_val
        //      -- store cycle_count and current_xval in x_vals
        //      -- pop execution_buffer
        // check if it is a noop or addx
        // if noop do nothing 
        // if addx insert (cycle_count + 2, amount) into execution_buffer
        // if let Some((cycle, amount)) = execution_buffer.get(0) {
        //     if cycle == &cycle_count {
        //         current_x_val += *amount;
        //         x_vals.insert(cycle_count, v)
        //     }
        // }
        
        match instruction {
            Instructions::NoOp => {
                x_vals.insert(cycle_count, current_x_val);
                cycle_count += 1;
                screen.write_to_position_and_advance(&sprite)
                
            },
            Instructions::Command { amount } => {
                let cycle_finished = cycle_count + 2;
                for i in cycle_count..cycle_finished {
                    x_vals.insert(i, current_x_val);
                    screen.write_to_position_and_advance(&sprite);
                }
                current_x_val += amount;
                sprite.add(amount);
                cycle_count += 2;
                x_vals.insert(cycle_count, current_x_val);
            },
        }

        
    }
    
    for i in 1..x_vals.len(){
        let val = x_vals.get_key_value(&i);
        println!("x_val at cycle: {i} = {val:?}");
    }

    let signal_strength_vals = vec![x_vals.get_key_value(&20_usize).unwrap(),x_vals.get_key_value(&60_usize).unwrap(), x_vals.get_key_value(&100_usize).unwrap(), x_vals.get_key_value(&140_usize).unwrap(), x_vals.get_key_value(&180_usize).unwrap(), x_vals.get_key_value(&220_usize).unwrap()];

    let signal_strength: i32 = signal_strength_vals.iter().map(|(&cycle_num, &x_val)| cycle_num as i32 * x_val).sum();


    println!("signal_strength = {signal_strength}\n\n");


    println!("screen : \n{screen}");




    
    println!("executed in {}mc", now.elapsed().as_micros());
}

pub enum Instructions {
    NoOp,
    Command {
        amount: i32
    }

}

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            Some(a) => {
                if a.0 == "addx" {
                    Ok(Instructions::Command { amount: a.1.parse()? })
                } else {
                    Err(anyhow::anyhow!("Failed to parse the instruction"))
                }
            },
            None => Ok(Instructions::NoOp),
        }

    }
}

struct Sprite {
    start_pos: i32,
    end_pos: i32,
    mid_pos: i32,
}

impl Sprite {
    pub fn new() -> Self {
        Self { start_pos: 0, mid_pos: 1, end_pos: 2 }
    }
    pub fn add(&mut self, action: &i32) {
        
        // if action.is_negative() {
        //     let new_action =  action.unsigned_abs() as usize;
        //     self.start_pos -= new_action;
        //     self.mid_pos -= new_action;
        //     self.end_pos -= new_action;
        // } else {
        //     let new_action =  action.abs() as usize;
        //     self.start_pos += new_action;
        //     self.mid_pos += new_action;
        //     self.end_pos += new_action;
        // }
        self.start_pos += action;
        self.mid_pos += action;
        self.end_pos += action;
    }

    pub fn write(&self, current_screen_pos: usize) -> char {
        if current_screen_pos as i32 >= self.start_pos && current_screen_pos as i32 <= self.end_pos {
            '#'
        } else {
            '.'
        }
    }
}

struct Screen {
    screen: [[char; 40]; 6],
    write_position: Position
}

impl Screen {
    pub fn new() -> Self {
        Self {
            screen: [[' '; 40]; 6],
            write_position: Position::default(),
        }
    }
    pub fn write_to_position_and_advance(&mut self, sprite: &Sprite) {
        
        self.screen[self.write_position.row][self.write_position.column] = sprite.write(self.write_position.column);
        if self.write_position.column == self.screen[0].len() - 1 {
            self.write_position.advance_row();
        } else {
            self.write_position.column += 1;
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.screen.iter() {
            let screen_str: String = row.iter().collect();
            write!(f,"{}\n",screen_str)?
            
        }
        Ok(())
    }
}



#[derive(Default)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn advance_row(&mut self) {
        self.row += 1;
        self.column = 0;
    }
}