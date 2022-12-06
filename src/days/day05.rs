type Instruction = (usize, usize, usize);

pub fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    // Traverse "stack lines" from last to first so:
    // - We know the number of stacks
    // - We can build a LIFO stack (i. e. element in top-most line is the last one to be inserted in the stack)
    let stacks_lines: Vec<&str> = input.rsplit('\n').collect();

    // Obtain the number of stacks by looking at the last digit
    let mut num_stacks: usize = 0; // Shouldn't be needed, because according to the input there will always be digits in the "first" line
    for ch in stacks_lines[0].chars().rev() {
        if ch.is_ascii_digit() {
            num_stacks = usize::try_from(ch.to_digit(10).unwrap()).unwrap();
            break;
        }
    }

    for (line_index, line) in stacks_lines.into_iter().enumerate() {
        for stack_index in 0..num_stacks {
            if line_index == 0 {
                // For the first line, we just want to create the stacks
                stacks.push(Vec::new());
            } else {
                // Otherwise, add elements if they are "valid"
                // First stack element appears at index 1 and then there are 3 formatting chars until the next one
                let element_index: usize = 4 * stack_index + 1;
                let element = line.chars().nth(element_index);
                match element {
                    None => continue, // Nothing found, line has ended, continue to next one
                    Some(' ') => (),  // Whitespace means no element
                    Some(element) => stacks[stack_index].push(element), // Any other character is an element
                }
            }
        }
    }

    stacks
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .map(|x| {
            let instruction_parts: Vec<&str> = x.split(' ').collect();
            (
                instruction_parts[1].parse::<usize>().unwrap(),
                instruction_parts[3].parse::<usize>().unwrap(),
                instruction_parts[5].parse::<usize>().unwrap(),
            )
        })
        .collect()
}

pub fn parse_input(input: String) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let stacks = parse_stacks(parts[0]);
    let instructons = parse_instructions(parts[1]);

    (stacks, instructons)
}

pub fn solve_a(input: String) -> Result<String, String> {
    let parsed_input = parse_input(input);
    let mut stacks = parsed_input.0;
    let instructions = parsed_input.1;

    for instruction in instructions {
        let num = instruction.0;
        let from = instruction.1 - 1;
        let to = instruction.2 - 1;

        for _ in 0..num {
            let element = stacks[from].pop().unwrap();
            stacks[to].push(element);
        }
    }

    let res: String = stacks.into_iter().map(|x| *x.last().unwrap()).collect();

    Ok(res)
}

pub fn solve_b(input: String) -> Result<String, String> {
    let parsed_input = parse_input(input);
    let mut stacks = parsed_input.0;
    let instructions = parsed_input.1;

    for instruction in instructions {
        let num = instruction.0;
        let from = instruction.1 - 1;
        let to = instruction.2 - 1;

        let mut to_move: Vec<char> = Vec::new();

        for _ in 0..num {
            let element = stacks[from].pop().unwrap();
            to_move.push(element);
        }

        for element in to_move.into_iter().rev() {
            stacks[to].push(element);
        }
    }

    let res: String = stacks.into_iter().map(|x| *x.last().unwrap()).collect();

    Ok(res)
}
