// For now, just a simple program that takes day and part from CLI args,
// reads the input from file, runs the program with the input and prints results
fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        // 3 args expected: advent_of_code_2022 <day> <part>
        3 => {
            let day = match args[1].parse::<u8>() {
                Ok(value) if !(1..26).contains(&value) => {
                    eprintln!("Error parsing day \"{}\": Not a valid day", value);
                    return;
                }
                Err(_) => {
                    eprintln!("Error parsing day \"{}\": Not a valid number", args[1]);
                    return;
                }
                Ok(value) => value,
            };

            // let day: Result<u8, ParseIntError> = args[1].parse();
            let part = match &args[2] {
                value if value == "a" || value == "b" => value,
                _ => {
                    eprintln!("Error parsing part \"{}\": Not a valid part", args[2]);
                    return;
                }
            };

            match solve(day, part) {
                Ok(solution) => println!("{}", solution),
                Err(error) => eprintln!("{}", error),
            };
        }
        _ => {
            help();
        }
    }
}

// Print usage
fn help() {
    println!("usage: advent_of_code_2022 <day> <part>");
}

// Solve a given part of a given day
// Includes obtaining the input file for the day, calling the solve method of the part of the day and returning the result
fn solve(day: u8, part: &String) -> Result<String, String> {
    // Obtain file and read from it
    let input = match std::fs::read_to_string(format!("./input/{:0>2}.txt", day)) {
        Ok(content) => content,
        Err(_) => return Err("Error reading input file".to_string()),
    };

    Ok(input)
}
