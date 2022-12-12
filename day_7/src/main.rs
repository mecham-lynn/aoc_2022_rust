use std::{time::Instant, collections::HashMap, path::PathBuf};

use helpers::AocArgs;

fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };

    // this needs to be a map of dir paths
    let mut dir_map: HashMap<String, usize> = HashMap::new();

    let terminal_lines = input_txt.lines();
    let mut dir_path = PathBuf::new();
    let mut current_dir = "";
    // let mut moves = vec![];
    for line in terminal_lines {
        
        
        let terminal_output = line.split_ascii_whitespace().collect::<Vec<&str>>();
        // println!("{:?}", &terminal_output);
        if terminal_output[0].contains("$") {
            let command = terminal_output[1];
            match command {
                "cd" => {
                    
                    let dir = terminal_output[2];
                    if dir == ".." {
                        dir_path.pop();
                        // println!("Moving to {}",dir_path.display());
                        current_dir = match dir_path.file_name() {
                            Some(a) => a.to_str().unwrap(),
                            None => "/",
                        }
                    } else {
                        dir_path.push(dir);
                        // println!("Moving to {}",dir_path.display());
                        current_dir = match dir_path.file_name() {
                            Some(a) => a.to_str().unwrap(),
                            None => "/",
                        }
                    }
                },
                "ls" => {
                    // println!("listing contents of {}", dir_path.display());
                }
                a => panic!("unknown command {a} found")
            }
        } else {
            // println!("current_dir {current_dir}");

            if terminal_output[0].contains("dir") {
                let mut new_dir = dir_path.clone();
                new_dir.push(terminal_output[1]);
                let dir_string = new_dir.as_path().to_str().unwrap().to_string();
                if !dir_map.contains_key(&dir_string) {
                    dir_map.insert(dir_string, 0);
                }
            } else {
                let size = terminal_output[0].parse().unwrap();
                for full_path in dir_path.ancestors() {

                    let dir_string = full_path.to_str().unwrap().to_string();
                    dir_map.entry(dir_string).and_modify(|a| {*a += size}).or_insert(size);
                }
            }
        }
    }

    if args.part_one{
    let mut result = dir_map.iter().map(|a| *a.1).filter(|b| b < &100_000).collect::<Vec<_>>();


    println!("result = {:?}", result.sort());
    let total_bytes: usize = result.iter().sum();

    println!("total_bytes = {total_bytes}");

    }

    if args.part_two {
        let total_storage_used = dir_map.get("/").unwrap();
        let max_storage = 70000000_usize;
        let needed_storage_available = 30000000_usize;

        let total_unused_storage = &max_storage - total_storage_used;

        let result = dir_map.iter().filter(|(_, &size)| size + total_unused_storage >= needed_storage_available).collect::<Vec<(_,_)>>();
        println!("smallest dirs = {:?}", result);
        let min = result.iter().reduce(|accum, item| {
            if accum.1 > item.1 {
                item
            } else {
                accum
            }
        }).unwrap();

        println!("dir to move = {:?}", min);
    }
    

    println!("executed in {}mc", now.elapsed().as_micros());

}
