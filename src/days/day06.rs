use std::collections::HashSet;

pub fn solve_a(input: String) -> Result<String, String> {
    let buffer = input.chars().collect::<Vec<char>>();
    let windows = buffer.windows(4);

    for (i, window) in windows.enumerate() {
        // To check if all chars are different, put them in a set and check the length - if 4 they are all different
        let char_set: HashSet<char> = window.iter().copied().collect();

        if char_set.len() == 4 {
            // The return should be the end of the string - not the start
            return Ok((i + 4).to_string());
        }
    }

    Err("No solution found".to_string())
}

pub fn solve_b(input: String) -> Result<String, String> {
    let buffer = input.chars().collect::<Vec<char>>();
    let windows = buffer.windows(14);

    for (i, window) in windows.enumerate() {
        // To check if all chars are different, put them in a set and check the length - if 14 they are all different
        let char_set: HashSet<char> = window.iter().copied().collect();

        if char_set.len() == 14 {
            // The return should be the end of the string - not the start
            return Ok((i + 14).to_string());
        }
    }

    Err("No solution found".to_string())
}
