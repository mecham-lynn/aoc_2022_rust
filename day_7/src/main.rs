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
                        println!("Moving to {}",dir_path.display());
                        current_dir = match dir_path.file_name() {
                            Some(a) => a.to_str().unwrap(),
                            None => "/",
                        }
                    } else {
                        dir_path.push(dir);
                        println!("Moving to {}",dir_path.display());
                        current_dir = match dir_path.file_name() {
                            Some(a) => a.to_str().unwrap(),
                            None => "/",
                        }
                    }
                },
                "ls" => {
                    println!("listing contents of {}", dir_path.display());
                }
                a => panic!("unknown command {a} found")
            }
        } else {
            println!("current_dir {current_dir}");
            let size = match terminal_output[0].parse::<usize>() {
                Ok(a) => a,
                Err(_) => continue,
            };
            // let filename = terminal_output[1];            
            // move up the dir_path until we hit / and iterate the size of each dir
            for full_path in dir_path.ancestors() {
                let current = match full_path.file_name() {
                    Some(a) => a.to_str().unwrap().to_owned(),
                    None => "/".to_owned(),
                };
                dir_map.entry(current).and_modify(|a| {*a += size}).or_insert(size);
            }
            
        }
    }
    // let test_vec = vec![

    // 274615
    // ,220692
    // ,21309
    // ,12989
    // ,46352
    // ,307491
    // ,164053
    // ,144223
    // ,274358
    // ,573
    // ,298079
    // ,33689
    // ,287144
    // ,164244
    // ,52508
    // ,195017
    // ,64762
    // ,56148
    // ,28260
    // ,91675
    // ,205543
    // ,37910
    // ,290553
    // ,175411
    // ,73620
    // ,125475
    // ,106099
    // ,136746
    // ,8406
    // ,55902
    // ,269256];

    // println!("{}", test_vec.iter().sum::<i32>());
    println!("{:?}", &dir_map);

    let mut result = dir_map.iter().map(|a| *a.1).filter(|b| b < &100_000).collect::<Vec<_>>();


    println!("result = {:?}", result.sort());
    let total_bytes: usize = result.iter().sum();

    println!("total_bytes = {total_bytes}");

    println!("executed in {}mc", now.elapsed().as_micros());

}
