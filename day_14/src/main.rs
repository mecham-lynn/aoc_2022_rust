use helpers::AocArgs;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::thread::current;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };
    // let input_txt = include_str!("../input-demo.txt");

    // Sand always starts at 500,0
    // Sand will always fall down 1 space
    // If blocked Sand will move diagonally down left 1 space
    // If blocked again Sand will move diagonally down right 1 space
    // If blocked again Sand is now "at rest"
    //
    // Blocked = when next space is sand or wall

    let walls: Vec<Wall> = input_txt
        .split("\n")
        .map(|a| {
            a.split(" -> ")
                .map(|b| b.split_once(",").map(|(x, y)| Point::new(x, y)).unwrap())
                .collect()
        })
        .collect();
    let mut room: Room = vec![];
    for wall in walls {
        expand_wall(wall)
            .iter()
            .for_each(|a| room.push((a.clone(), FillType::Wall)))
    }
    
    room.sort_by(|a,b |a.0.partial_cmp(&b.0).unwrap());
    
    let (min_x, max_x, max_y) = find_view_bounds(&room);
    println!("min_x: {min_x}, max_x: {max_x}, max_y: {max_y}");
    
    let mut current_sand = Sand::new();
    let mut sand_units = 0;
    
    //TODO: if part_two add a wall at the max_y value all the way across
    

    loop {
        current_sand.fall(&room);
        match current_sand.state {
            SandState::Falling => if current_sand.current_location.y >= max_y && !args.part_two {
                break
            },
            SandState::Resting => {
                // if part_two only break when sand is at it's starting location (blocking the entrance)
                if args.part_two {
                    if current_sand.current_location == Point::new_from_num(500, 0) {
                        break;
                    }
                }
                
                // add sand location to room
                room.push((current_sand.current_location.clone(), FillType::Sand));
                // sort room so binary search works
                room.sort_by(|a,b |a.0.partial_cmp(&b.0).unwrap());
                // start a new sand particle falling
                current_sand = Sand::new();
                // increment the count of particles
                sand_units += 1;
            },
        }
    }
        
    
    print_room(&room);
    
    println!("Number of sand units before dropping into the abys: {}", sand_units);
    println!("executed in {}mc", now.elapsed().as_micros());
    
}

type Room = Vec<(Point, FillType)>;
type Wall = Vec<Point>;

fn print_room(room: &Room) {
    let mut file =  BufWriter::new(File::create("wall_display.txt").unwrap());
    let (min_x, max_x, max_y) = find_view_bounds(room);
    // let mut cursor = Point::new_from_num(min_x,0);
    
    for y in 0..=max_y {
        for x in min_x..=max_x {
            let seek = Point::new_from_num(x, y);
            match room.binary_search_by(|a| a.0.partial_cmp(&seek).unwrap()) {
                Ok(index) => {
                    // println!("Found {seek:?}");
                    let point = &room[index];
                    match point.1 {
                        FillType::Sand => {file.write(b" +").unwrap();},
                        FillType::Wall => {file.write(b" #").unwrap();},
                    }
                },
                Err(_) =>{
                    // println!("Couldn't find {seek:?}");
                    file.write(b"  ").unwrap();
                },
            }
            // match room.iter().find(|(a)|a.0) {
                
            // }
        }
        file.write(b"\n").unwrap();
    }
}

fn find_view_bounds(room: &Room) -> (usize, usize, usize){
    let mut min_x = 10_000_usize;
    let mut max_x = 0_usize;
    let mut max_y: usize = 0_usize;
    for (point, _fill_type) in room {
        
        // If the current point is less than the min set min to that number rounded down to the nearest 100
        if point.x < min_x {
            let min_un_rounded = point.x as f64;
            let rounded = (min_un_rounded / 10.0).floor() * 10.00;
            
            min_x = rounded as usize;
        }
        
        if point.x > max_x {
            max_x = round_val_up(point.x);
        }
        
        if point.y > max_y {
            max_y = point.y;
        }
    }
    
    // println!("min_x: {min_x}, max_x: {max_x}, max_y: {max_y}");
    
    (min_x, max_x, max_y + 2)
}

fn round_val_up(num: usize) -> usize {
    if num < 10 {
        return 10
    }
    let quotient = num/10;
    let remainder = num %10;
    
    if remainder == 0 {
        num
    } else {
        (quotient + 1) * 10
    }
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: &str, y: &str) -> Self {
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
    fn new_from_num(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FillType {
    Sand,
    Wall,
}

fn expand_wall(wall: Wall) -> Wall {
    let mut new_wall = vec![];
    let mut previous_corner: Option<Point> = None;
    for corner in wall {
        match previous_corner {
            Some(p_corner) => {
                if p_corner.x == corner.x && p_corner.y != corner.y {
                    // Expand the y axis
                    // println!("expanding y axis");
                    let (min_y, max_y) = if p_corner.y <= corner.y {
                        (p_corner.y, corner.y)
                    } else {
                        (corner.y, p_corner.y)
                    };
                    for new_y in min_y..max_y {
                        // println!("new_y = {new_y}");
                        
                        new_wall.push(Point::new_from_num(p_corner.x, new_y))
                    }
                } else if p_corner.y == corner.y && p_corner.x != corner.x {
                    // println!("expanding x axis");
                    // println!("p_corner.x {}, corner.x {} ",p_corner.x,corner.x);
                    
                    let (min_x, max_x) = if p_corner.x <= corner.x {
                        (p_corner.x, corner.x)
                    } else {
                        (corner.x, p_corner.x)
                    };
                    
                    
                    // expand the x axis
                    for new_x in min_x..max_x {
                        // println!("new_x = {new_x}");
                        let new_point = Point::new_from_num(new_x, p_corner.y);
                        // println!("new point {:?}", &new_point);
                        new_wall.push(new_point);
                    }
                } else {
                    panic!("you cannot have two corners matching each other.... previous:{p_corner:?} current:{corner:?}");
                }
                
                new_wall.push(corner.clone());
                previous_corner = Some(corner)
                
            },
            None => {
                new_wall.push(corner.clone());
                previous_corner = Some(corner)
            },
        }
    }
    
    new_wall
}


enum FallDirection {
    Down,
    DiagRight,
    DiagLeft,
}

enum SandState {
    Falling,
    Resting,
}

pub struct Sand {
    current_location: Point,
    state: SandState,
}

impl Sand {
    pub fn new() -> Self {
        Self {
            current_location: Point::new("500", "0"),
            state: SandState::Falling,
        }
    }
    
    fn get_down_point(&self) -> Point {
        Point::new_from_num(self.current_location.x, self.current_location.y + 1)
    }
    
    fn get_down_left(&self) -> Point {
        Point::new_from_num(self.current_location.x -1, self.current_location.y + 1)
    }
    
    fn get_down_right(&self) -> Point {
        Point::new_from_num(self.current_location.x + 1, self.current_location.y + 1)
    }
    
    pub fn fall(&mut self, room: &Room) {
        // look down one space
        // if that space is taken
        // then look down right
        // 
        
        let down = &self.get_down_point();
        let down_left = &self.get_down_left();
        let down_right = &self.get_down_right();
        
        // look down
        match room.binary_search_by(|(point, _fill)| point.partial_cmp(down).unwrap()) {
            // look down left
            Ok(_) => match room.binary_search_by(|(point, _fill)| point.partial_cmp(down_left).unwrap()) {
                // look down right
                Ok(_) => match room.binary_search_by(|(point, _fill)| point.partial_cmp(down_right).unwrap()) {
                    Ok(_) => {
                        self.state = SandState::Resting;
                    },
                    Err(_) => {
                        self.current_location = down_right.clone();
                    },
                },
                Err(_) => {
                    self.current_location = down_left.clone();
                },
            },
            Err(_) => {
                self.current_location = down.clone();
            },
        }
    }
}

