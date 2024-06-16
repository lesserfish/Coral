extern crate coral;
use std::env;
mod frontend;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Error: No input provided. Please specify the path to a .nes file.");
        std::process::exit(-1);
    }

    let filepath = args[1].clone();
    frontend::main(filepath);
}
