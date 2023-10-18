use std::{time::Instant, collections::HashMap};

use helpers::AocArgs;

fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        println!("Demo text enabled");
        include_str!("../input-demo.txt")
    } else {
        println!("Demo text disabled");
        include_str!("../input.txt")
    };

    
    let mut current_round = 0;
    let mut monkeys =  Monkey::new(input_txt);
    let mut monkey_activity: HashMap<usize, usize> = HashMap::new();

    let (max_rounds, part_two) = if args.part_two {
        let modulus: u64 = monkeys.iter().map(|(_, m)| m.test).product();
        (10_000, Some(modulus))
    } else {
        (20, None)
    };


    while current_round < max_rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(&i).unwrap();

            let targets = monkey.take_turn(&mut monkey_activity, part_two);
            for (target_monkey_id, item) in targets {
                // throw the item to the other monkey
                monkeys.entry(target_monkey_id).and_modify(|a| a.starting_items.push(item));
            }
        }
        current_round += 1;
    }

    let mut monkey_activities: Vec<usize> = monkey_activity.iter().map(|(_, &b)|b).collect();

    monkey_activities.sort_by(|a,b| b.cmp(a));
    println!("Monkey activities: {monkey_activities:?}");

    println!("monkey activity score {}", monkey_activities[0] * monkey_activities[1]);



    println!("executed in {}mc", now.elapsed().as_micros());
}

struct Monkey {
    id: usize,
    starting_items: Vec<u64>,
    operation: OperationType,
    test: u64,
    targets: (usize, usize)
}
impl Monkey {
    pub fn new(args: &str) -> HashMap<usize, Monkey> {
        let mut monkeys: HashMap<usize, Monkey> = HashMap::new();

        for (monkey_index, monkey_args) in args.split("\n\n").enumerate() {
            // don't care about the first line since the monkeys are in order
            let mut args = monkey_args.lines().skip(1);

            let items = args.next()
                .unwrap()[18..]
                .split(", ")
                .filter_map(|a| a.parse::<u64>().ok())
                .collect::<Vec<u64>>();
            
            let operation = OperationType::from(args.next()
                .unwrap()[23..]
                .split_once(" ").unwrap());
            
            let test = args
                .next()
                .unwrap()
                .split_once("by ")
                .unwrap()
                .1
                .parse::<u64>()
                .unwrap();

            let target_if_true = args
                .next()
                .unwrap()
                .split_once("monkey ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();
            
            let target_if_false = args
                .next()
                .unwrap()
                .split_once("monkey ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();
                



            monkeys.insert(monkey_index, Monkey { id: monkey_index, starting_items: items, operation, test, targets: (target_if_true, target_if_false) });

        }


        monkeys
    }

    fn take_turn(&mut self, monkey_activity: &mut HashMap<usize, usize>, part_two: Option<u64>) -> Vec<(usize, u64)> {
        
        let mut actions = vec![];
        for item in self.starting_items.iter_mut() {
            // do the operation
            match self.operation {
                OperationType::Plus(a) => *item += a,
                OperationType::Multiplied(a) => *item *= a,
                OperationType::Square => *item = item.pow(2),
            }

            match part_two {
                // modulus by the product of all of the test numbers for all monkeys
                Some(a) => *item %= a,
                // divide the result of operation by 3
                None => *item /= 3,
            }
            
            

            // do the test
            if *item % self.test == 0 {
                actions.push((self.targets.0, *item))
            } else {
                actions.push((self.targets.1, *item))
            }
            monkey_activity.entry(self.id).and_modify(|a| *a += 1).or_insert(1);

        }



        self.starting_items.clear();
       
        actions

    }
}

// struct Operation {
//     val: i32,
//     operation_type: OperationType,
//     constant: i32,
// }

// impl Operation {
//     pub fn new(args: [&str; 3]) -> Self {

//     }
// }

// impl Operation {
//     pub fn new()
// }

enum OperationType {
    Plus(u64),
    Multiplied(u64),
    Square,
}

impl OperationType {
    fn from((op, num): (&str, &str)) -> Self { 
        match op {
            "+" => {
                // parse the num. if we get an error this means that we will be adding by itself
                // which is the same as multiplying by 2
                match num.parse::<u64>() {
                    Ok(a) => Self::Plus(a),
                    Err(_) => Self::Multiplied(2),
                }
            },
            "*" => {
                // parse the num. if we get an error that means we will be multiplying by itself
                match num.parse::<u64>() {
                    Ok(a) => Self::Multiplied(a),
                    Err(_) => Self::Square,
                }
            },
            a => unreachable!(" failed to parse operation '{a}'. This value is not supported")
        }
    }
}

