pub fn solve_a(input: String) -> Result<String, String> {
    let instructions: Vec<&str> = input.split('\n').collect();
    let mut x = 1;
    let mut res = 0;

    let mut current_instruction_index = 0;

    // In the first cycle the first cycle, the first instruction is already "being done", so start this "counter"
    // with the value from the first instruction
    let mut current_instruction_cycles_left =
        match instructions[0].split(' ').collect::<Vec<&str>>()[0] {
            "noop" => 1,
            "addx" => 2,
            _ => 0, // Shouldn't happen according to problem statement
        };

    // Starting cycles at 1 to make it easier to pass from "nth cycle" on the problem statement to cycle on the code
    for current_cycle in 1..221 {
        // At the start of the cycle - add to sum if on an "interesting" cycle
        match current_cycle {
            20 | 60 | 100 | 140 | 180 | 220 => res += current_cycle * x,
            _ => (),
        }

        // Decrease number of cycle until instruction ends
        current_instruction_cycles_left -= 1;

        // If the instruction is ending - do its function and move to the next one
        if current_instruction_cycles_left == 0 {
            let instruction = instructions[current_instruction_index];
            let parts: Vec<&str> = instruction.split(' ').collect();
            let command = parts[0];
            let value: i32 = parts.get(1).unwrap_or(&"0").parse().unwrap();

            // Do command - only addx actually does something
            if command == "addx" {
                x += value
            }

            // Move to the next one - and calculate cycles left based on that instruction
            current_instruction_index += 1;

            current_instruction_cycles_left = match instructions[current_instruction_index]
                .split(' ')
                .collect::<Vec<&str>>()[0]
            {
                "noop" => 1,
                "addx" => 2,
                _ => 0, // Shouldn't happen according to problem statement
            };
        }
    }

    Ok(res.to_string())
}

pub fn solve_b(input: String) -> Result<String, String> {
    let instructions: Vec<&str> = input.split('\n').collect();

    let mut current_instruction_index = 0;

    let mut current_instruction_cycles_left =
        match instructions[0].split(' ').collect::<Vec<&str>>()[0] {
            "noop" => 1,
            "addx" => 2,
            _ => 0, // Shouldn't happen according to problem statement
        };

    let mut x = 1;
    let mut res = "".to_string();
    let mut current_row = "".to_string();
    let mut current_cycle = 1;

    loop {
        // At the start of the cycle - draw pixel
        let current_pixel_index = (current_cycle - 1) % 40; // Cycles start at 1 - Pixels index at 0
        if (x - 1..x + 2).contains(&current_pixel_index) {
            current_row.push('#');
        } else {
            current_row.push('.');
        };

        // If end of row - reset current row
        if current_cycle % 40 == 0 {
            res = format!("{res}\n{current_row}");
            current_row = "".to_string();
        }

        current_instruction_cycles_left -= 1;

        if current_instruction_cycles_left == 0 {
            let instruction = instructions[current_instruction_index];
            let parts: Vec<&str> = instruction.split(' ').collect();
            let command = parts[0];
            let value: i32 = parts.get(1).unwrap_or(&"0").parse().unwrap();

            if command == "addx" {
                x += value
            }

            current_instruction_index += 1;

            if current_instruction_index >= instructions.len() {
                break;
            }

            current_instruction_cycles_left = match instructions[current_instruction_index]
                .split(' ')
                .collect::<Vec<&str>>()[0]
            {
                "noop" => 1,
                "addx" => 2,
                _ => 0, // Shouldn't happen according to problem statement
            };
        }

        // Move to the next cycle
        current_cycle += 1;
    }

    Ok(res)
}
