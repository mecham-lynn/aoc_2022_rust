use std::{time::Instant};

use helpers::AocArgs;
fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };

    // get the tree heights into a 



    println!("executed in {}mc", now.elapsed().as_micros());
}
