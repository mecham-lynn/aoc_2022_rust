use helpers::AocArgs;
use regex::Regex;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };
    
    let valves: Vec<Valve> = input_txt.lines().map(|a| Valve::new(a).unwrap()).collect();
    

    println!("executed in {}mc", now.elapsed().as_micros());
}

struct Valve {
    name: String,
    flow_rate: usize,
    connected_valves: Vec<String>,
}

impl Valve {
    pub fn new(line: &str) -> Result<Self, String> {
        let re = Regex::new(r"^Valve (..) .* rate=(\d+); .*valves? (.*)$").unwrap();
        if let Some(capture) = re.captures(line) {
            return Ok(Self {
                name: capture[1].to_string(),
                flow_rate: capture[2].parse().unwrap(),
                connected_valves: capture[3].split(", ").map(|s| s.to_string()).collect(),
            })
        }
        Err("Failed to parse string".to_string())
    }
}
