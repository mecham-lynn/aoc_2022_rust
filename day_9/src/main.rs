use std::{time::Instant, str::FromStr, ops::{Sub, SubAssign, Add, AddAssign}, fmt::Display, hash::Hash, collections::{HashSet, HashMap}};
use anyhow::anyhow;

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

    let commands: Vec<Command> = input_txt.lines().flat_map(|f| f.split_once(" ").map(|(d, a)| Command::new(d, a).unwrap())).collect();

    // let mut rope = Rope::new();
    let mut rope: HashMap<usize, Position> = if args.part_two{
            (0_usize..10).into_iter().map(|a| (a, Position::new())).collect()
        } else {
            HashMap::from([(0, Position::new()), (1, Position::new())])
        };
    let mut tail_positions: HashSet<Position> = HashSet::new();

    for command in &commands {
        execute_command(&mut rope, command, &mut tail_positions)
    }

    println!("\n\nall visited tail positions {:?}\n\n", tail_positions);
    // println!("unique visited tail positions {:?}\n\n", tail_positions);
    println!("number of positions tail visited {}", tail_positions.iter().count());





    println!("executed in {}mc", now.elapsed().as_micros());
}

#[derive(Debug, Clone, Copy)]
pub struct Command {
    direction: Direction,
    amount: usize,
}
impl Command {
    fn new(direction: &str, amount: &str) -> anyhow::Result<Self> {
        let direction = Direction::from_str(direction)?;
        let amount = amount.parse()?;

        Ok(Self {
            direction,
            amount,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "r" => Ok(Self::Right),
            "l" => Ok(Self::Left),
            "u" => Ok(Self::Up),
            "d" => Ok(Self::Down),
            a => Err(anyhow!("unknown direction '{a}' detected"))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
    fn get_distance_between_two_points(&self, other: &Self) -> i32 {
        // println!("sub result {}", result);
        // (result.x - result.y).abs()
        let result = f32::sqrt((self.x as f32 - other.x as f32).powi(2) + (self.y as f32 - other.y as f32).powi(2));
        result.abs() as i32

    }
}


impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"x: {}, y: {}", self.x, self.y)
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl From<&Direction> for Position {
    fn from(dir: &Direction) -> Self {
        match dir {
            Direction::Right => Self { x: 1, y: 0 },
            Direction::Up => Self { x: 0, y: 1 },
            Direction::Left => Self { x: -1, y: 0 },
            Direction::Down => Self { x: 0, y: -1 },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rope {
    head: Position,
    tail: Position
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"head_pos: {}, tail_pos: {}", self.head, self.tail)
    }
}

impl Rope {
    pub fn new() -> Self {
        Self {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
        }
    }
    /// Changes the postion of the Head and Tail of the rope
    fn execute_command(&mut self, command: &Command, tail_positions: &mut HashSet<Position>){
        println!("Command {:?}", &command);

        //move in the direction for the number of times in the command
        let offset = Position::from(&command.direction);
        for _dir_amount in 1..=command.amount {
            let orig_head = self.head;
            self.head += offset;

            // println!("distance between head and tail {}", self.get_distance_between_head_and_tail());
            if self.get_distance_between_head_and_tail() > 1 {
                
                self.tail = orig_head
                
            }
            // println!("Tail is at position {}", self.tail);
            tail_positions.insert(self.tail);
        }

    }

    fn get_distance_between_head_and_tail(&self) -> i32 {
        // println!("sub result {}", result);
        // (result.x - result.y).abs()
        let result = f32::sqrt((self.head.x as f32 - self.tail.x as f32).powi(2) + (self.head.y as f32 - self.tail.y as f32).powi(2));
        result.abs() as i32

    }

}

pub fn execute_command(rope: &mut HashMap<usize, Position>, command: &Command, tail_positions: &mut HashSet<Position>) {
    // println!("Command {:?}", &command);

    let offset = Position::from(&command.direction);
    for _dir_amount in 1..=command.amount{
        let mut previous_pos = rope[&0];
       
        for i in 0..rope.len() - 1 {
            let front_pos = rope[&i];
            // let orig_front_pos = *front_pos;

            
            let tail_index = rope.len() - 1;
            let tail_pos = rope[&tail_index];
            
            let next_pos = rope.get(&{i + 1}).unwrap();
            let new_front_pos = if i == 0 {
                front_pos.clone() + offset
            } else {
                previous_pos
            };

            previous_pos = front_pos.clone();

            

            if new_front_pos.get_distance_between_two_points(next_pos) > 1 {
                // if i == 0 {
                //     // only the head should move using the offset
                //     rope.entry(0).and_modify(|a| {*a += offset});
                // }

                rope.entry(i + 1).and_modify(|entry| *entry = previous_pos);
                
                
            }

            rope.entry(i).and_modify(|entry| *entry = new_front_pos);


            tail_positions.insert(tail_pos);
            // let front_knot = rope.get_mut(&i).unwrap();
            // let orig_front_pos = *front_knot;
            // // let next_knot = rope.get_mut(&{i + 1}).unwrap();

            // if front_knot.get_distance_between_two_points(rope.get_many_mut(&{i + 1}).unwrap()) > 1 {
            //     rope.entry(i + 1).and_modify(|a| *a = orig_front_pos);
            // }
            
        }
    }
    // for _dir_amount in 1..=command.amount {

    // }

    //move in the direction for the number of times in the command
    // let offset = Position::from(&command.direction);
    // for _dir_amount in 1..=command.amount {
    //     let orig_head = self.head;
    //     self.head += offset;

    //     // println!("distance between head and tail {}", self.get_distance_between_head_and_tail());
    //     if self.get_distance_between_head_and_tail() > 1 {
            
    //         self.tail = orig_head
            
    //     }
    //     // println!("Tail is at position {}", self.tail);
    //     tail_positions.insert(self.tail);
    // }

}


