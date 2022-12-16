use std::{time::Instant, str::FromStr, ops::{Sub, SubAssign, Add, AddAssign}, fmt::Display, hash::Hash, collections::{HashSet, HashMap}};
use anyhow::anyhow;

use helpers::AocArgs;
fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };

    let commands: Vec<Command> = input_txt.lines().flat_map(|f| f.split_once(" ").map(|(d, a)| Command::new(d, a).unwrap())).collect();

    let mut rope = Rope::new();

    let mut tail_positions: HashSet<Position> = HashSet::new();

    for command in &commands {
        rope.execute_command(command, &mut tail_positions, args.part_one)
    }


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

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
struct Rope {
    head: Position,
    tail: [Position; 9]
}

impl Rope {
    pub fn new() -> Self {
        Self {
            head: Position { x: 0, y: 0 },
            tail: [Position::new(); 9],
        }
    }
    /// Changes the postion of the Head and Tail of the rope
    fn execute_command(&mut self, command: &Command, tail_positions: &mut HashSet<Position>, part_one: bool){
        // println!("Command {:?}", &command);

        //move in the direction for the number of times in the command
        let offset = Position::from(&command.direction);
        for _dir_amount in 1..=command.amount {
            // add the offset to the head
            self.head += offset;

            // trigger the next section of the rope to follow
           Self::follow(&self.head, &mut self.tail[0]);

           // insert the current position of the last rope segment (i.e the tail) to a HashSet<Position>
           if part_one {
                tail_positions.insert(self.tail[0]);
           } else {
                for i in 1..9 {
                    let (new_head, next_tail) = self.tail.split_at_mut(i);
                    Self::follow(&new_head[i - 1], &mut next_tail[0]);
                }
                tail_positions.insert(self.tail[8]);
           }
        }

    }

    /// Check the distance between the leading_pos and next_knot. 
    /// If it is greater than 0 we need to change the position of next_knot.
    fn follow(leading_pos: &Position, next_knot: &mut Position) {
        let dx = leading_pos.x - next_knot.x;
        let dy = leading_pos.y - next_knot.y;

        if (dx.abs() + dy.abs() > 1) && (dx.abs() > 1 || dy.abs() > 1) {
            next_knot.x += dx.signum();
            next_knot.y += dy.signum();
        }
    }

}




