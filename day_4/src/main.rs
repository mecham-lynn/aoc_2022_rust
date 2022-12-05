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

   

    let cleaning_assignment_pairs: Vec<(CleaningAssignment, CleaningAssignment)>= input_txt.split("\n")
    .map(|a| 
        {
            let elf_pair = a.split_once(",").unwrap();
            let elf_1 = elf_pair.0.split_once('-').unwrap();
            let elf_2 = elf_pair.1.split_once('-').unwrap();
            (CleaningAssignment::new(elf_1.0, elf_1.1), CleaningAssignment::new(elf_2.0, elf_2.1))

        }
    ).collect();
    if args.part_one {
        let overlaps:Vec<&(CleaningAssignment, CleaningAssignment)> = cleaning_assignment_pairs.iter()
        .filter(|(a, b)| a.contains(b))
        .collect();

        println!("Number of overlaps: {}", overlaps.len());
    }

    if args.part_two {
        
        let overlaps: Vec<&(CleaningAssignment, CleaningAssignment)> = cleaning_assignment_pairs.iter()
        .filter(|(a, b)| a.overlaps(b))
        .collect();

        // for overlap in &overlaps {
        //     println!("overlaps: {:?}", &overlap);
        // }
        println!("Number of overlaps: {}", overlaps.len());
    }

    



    println!("executed in {}mc", now.elapsed().as_micros());
}

#[derive(Debug)]
pub struct CleaningAssignment {
    start: i32, 
    end: i32
}
impl CleaningAssignment {
    fn new(start: &str, end:&str) -> Self {
        Self { start: start.parse().unwrap(), end: end.parse().unwrap() }
    }

    fn contains(&self, other: &Self) -> bool {
        (self.start <= other.start && self.end >= other.end) 
        || (other.start <= self.start && other.end >= self.end)
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.start ||
        other.start <= self.start && other.end >= self.start
    }
}