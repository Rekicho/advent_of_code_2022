// For now, just a simple program that takes day and part from CLI args,
// reads the input from file, runs the program with the input and prints results
use std::env;

fn help() {
    println!("usage: advent_of_code_2022 <day> <part>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // 3 args expected: <command>
        3 => {
            let day = &args[1];
            let part = &args[2];

            println!("Solving Day {}, Part {}", day, part);
        }
        _ => {
            help();
        }
    }
}
