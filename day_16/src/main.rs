use helpers::AocArgs;
use regex::Regex;
use std::{collections::HashMap, time::Instant};

fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };

    let valves: Vec<Valve> = input_txt.lines().map(|a| Valve::new(a).unwrap()).collect();
    let valve_map: HashMap<String, &Valve> = valves.iter().map(|a: &Valve| (a.name.clone(), a)).collect(); 

    let node = Node::new(&valves, &valve_map);

    println!("{node:#?}");

    println!("executed in {}mc", now.elapsed().as_micros());
}

//PARSING: flat HashMap of Nodes and their definitions
// After we are done creating the hashmap create the BIG NODE and add connections using that hashmap

// Create the tree
// Spawn a thread per decision (movement forward|back or activating a valve)
// dies at 30 "min/moves"

//thread Get's Decisions (open valve, go to next valve(s), go back to parent)
//Spawn new thread for all but one decision
//Take decision
//update variables

struct Worker {
    total_water: usize,
    water_per_minute: Vec<usize>,
    steps_taken: Vec<String>,
    valves_opened: Vec<String>,
}

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: usize,
    connected_valves: Vec<String>,
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    flow_rate: usize,
    connected_nodes: Vec<Node>,
}

impl Valve {
    pub fn new(line: &str) -> Result<Self, String> {
        let re = Regex::new(r"^Valve (..) .* rate=(\d+); .*valves? (.*)$").unwrap();
        match re.captures(line) {
            Some(captures) => {
                return Ok(Self {
                    name: captures[1].to_string(),
                    flow_rate: captures[2].parse().unwrap(),
                    connected_valves: captures[3].split(", ").map(|s| s.to_string()).collect(),
                })
            }
            None => panic!("NO MATCHES FROM REGEX"),
        }
    }
}

impl Node {
    pub fn new(valves: &[Valve], map: &HashMap<String, &Valve>) -> Self {

        let valve = valves.first().unwrap();
        Self::create_from_valve(valve, None, valve, map)
    }

    fn create_from_valve(start: &Valve, parent: Option<&Valve>, valve: &Valve, map: &HashMap<String, &Valve>) -> Self {
        println!("PARENT: {:?}", parent);
        println!("CURRENT VALVE: {}", valve.name);
        let connections: Vec<Node> = {
            let mut connections = vec![];
            for connection in &valve.connected_valves {
                let connection_valve_details = map.get(connection).unwrap();
                // if we have a parent and it matches the current name skip otherwise recurse!
                // if !parent.is_some() || &parent.unwrap().name != connection || connection != &start.name {
                //     // RECURSE!!!!!
                //     connections.push(Self::create_from_valve(start, Some(valve), &connection_valve_details, map))
                // } 
            }
            connections
        };
        Self {
            name: valve.name.clone(),
            flow_rate: valve.flow_rate,
            connected_nodes: connections,
        }
    }
}
