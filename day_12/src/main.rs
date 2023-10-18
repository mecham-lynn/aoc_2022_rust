use std::{time::Instant, collections::HashMap};
use pathfinding::prelude::bfs;

use helpers::AocArgs;

type Map = Vec<Vec<char>>;

fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };
    // let input_txt =  include_str!("../input-demo.txt");

    let letters = "abcdefghijklmnopqrstuvwxyz";
    let mut letter_map: HashMap<char, usize> = letters.chars().enumerate().map(|(num, c)| (c, num)).collect();
    letter_map.insert('S', 0);
    letter_map.insert('E',25);

    println!("Letter Map: {:?}", &letter_map);

    let height_map: Map = input_txt.lines().map(|a| a.chars().collect()).collect();

    let (start_pos, end_pos) = find_start_end(&height_map);

    println!("MAP: \n");
    for map in &height_map {
        let map_str: String = map.iter().collect();
        println!("{map_str}")
    }

    let solution = if !args.part_two {

        bfs(&start_pos, |p| p.next_moves(&height_map, &letter_map, false), |p| p == &end_pos)
            .expect("No path found")

    } else {
        bfs(&end_pos, |p| p.next_moves(&height_map, &letter_map, true), |p| height_map[p.y][p.x] == 'a')
            .expect("No path found")
    };

    println!("Solution Path: {:?}", &solution);
    println!("Solution length: {}", solution.len() - 1);



    println!("executed in {}mc", now.elapsed().as_micros());
}

fn find_start_end(map: &Map) -> (Position, Position) {
    let mut start_end = (Position::default(), Position::default());
    for (y, row) in map.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if char == &'S' {
                println!("found start position at ({x},{y})");
                start_end.0 = Position::new(x, y)
            }
            if char == &'E' {
                println!("found the end position at ({x},{y})");
                start_end.1 = Position::new(x, y)
            }
        }
    }
    start_end
}


// struct Location {
//     pos: Position,
//     value: char
// }

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,

}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl Position {
    fn next_moves(&self, map: &Map, letter_map: &HashMap<char, usize>, part_two: bool) -> Vec<Self> {

        let max_x = map[0].len() - 1;
        let max_y = map.len() - 1;
        // println!("max x = {max_x}, max y = {max_y}");

        let mut valid_pos = vec![];
        if self.x > 0 {
            // can check left
             let pos = self.get_next_pos_from_direction(Direction::Left);
            if self.is_direction_valid(&pos, &map, &letter_map, part_two) {
                valid_pos.push(pos);
            }
        }
        if self.x <= max_x {
            // can check right
            let pos = self.get_next_pos_from_direction(Direction::Right);
            if self.is_direction_valid(&pos, &map, &letter_map, part_two) {
                valid_pos.push(pos);
            }
        }
        if self.y > 0 {
            // can check Up
            let pos = self.get_next_pos_from_direction(Direction::Up);
            if self.is_direction_valid(&pos, &map, &letter_map, part_two) {
                valid_pos.push(pos);
            }
        }

        if self.y <= max_y {
            // can check Down
            let pos = self.get_next_pos_from_direction(Direction::Down);
            if self.is_direction_valid(&pos, &map, &letter_map, part_two) {
                valid_pos.push(pos);
            }
        }

        valid_pos
    }

    fn get_next_pos_from_direction(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self { x: self.x, y: self.y - 1},
            Direction::Down => Self {x: self.x, y: self.y + 1},
            Direction::Left => Self{x: self.x - 1, y: self.y},
            Direction::Right => Self{x: self.x + 1, y: self.y},
        }
    }

    fn is_direction_valid(&self, next_pos: &Self, map: &Map, letter_map: &HashMap<char, usize>, part_two: bool) -> bool {
        let max_x = map[0].len() - 1;
        let max_y = map.len() - 1;
        if next_pos.y > max_y || next_pos.x > max_x {
            false
        } else {
            let curr_char = &map[self.y][self.x];
            let next_char = &map[next_pos.y][next_pos.x];
            // println!("curr_char = {curr_char} at pos {:?}", &self);
            // println!("next_char = {next_char} at pos {:?}", &next_pos);

            let cur_char_height = letter_map.get(curr_char).unwrap();
            let next_char_height =  letter_map.get(next_char).unwrap();

            if !part_two {
                if next_char_height < cur_char_height {
                    true
                } else {
                    next_char_height - cur_char_height <= 1
                }
            } else {
                if next_char_height > cur_char_height {
                    true
                } else {
                    cur_char_height - next_char_height <= 1

                }
            }

            // letter_map.get(&map[next_pos.y][next_pos.x]).unwrap() - letter_map.get(&map[self.y][self.x]).unwrap() < 2
        }

    }

}


enum Direction {
    Up,
    Down,
    Left,
    Right,
}
// impl From<Direction> for Position {
//     fn from(value: Direction) -> Self {
//         match value {
//             // Direction::Up => Position(0, 1)
//             // Direction::Down => Position(0)
//             // Direction::Left => {}
//             // Direction::Right => {}
//         }
//     }
// }


