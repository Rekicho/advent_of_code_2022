use std::collections::HashMap;

// It would probably be more correct to represent the fily system as a tree,
// with each instance being a tuple of name and vector of "entries" (directory or file)
// but, for simplification, opted to represent it as a map
// in which each entry has the full directory path as key
// and the name and a vector of files (that are directly in the folder) as value
type File = (String, u32);
type Directory = (String, Vec<File>);

// Kinda got messy with '/'s by not assuming that the inital instruction is cd / and base all paths there
// but it works :')
fn build_directories(input: String) -> HashMap<String, Directory> {
    let instructions: Vec<&str> = input.split("$ ").collect();
    let mut directories: HashMap<String, Directory> = HashMap::new();
    let mut current_pwd = "".to_string();

    for instruction in instructions {
        // Ignore first instruction as, they way we are spliting, it will be empty
        if instruction.is_empty() {
            continue;
        }

        let parse_instruction = &instruction[..instruction.len() - 1]; // Remove last \n
        let parts: Vec<&str> = parse_instruction.splitn(2, '\n').collect();
        let input_parts: Vec<&str> = parts[0].split(' ').collect();

        match input_parts[0] {
            "cd" => match input_parts[1] {
                ".." => {
                    let last_slash_index = current_pwd.rfind('/').unwrap(); // Do not remove base /
                    if last_slash_index != 0 {
                        current_pwd = current_pwd.chars().take(last_slash_index).collect()
                    }
                }
                val => {
                    if current_pwd.is_empty() || current_pwd.ends_with('/') {
                        current_pwd.push_str(val);
                    } else {
                        current_pwd.push_str(&format!("/{val}")[..]);
                    }

                    if !directories.contains_key(&current_pwd) {
                        directories.insert(current_pwd.clone(), (val.to_string(), Vec::new()));
                    }
                }
            },
            "ls" => {
                let output_lines: Vec<&str> = parts[1].split('\n').collect();
                let mut files: Vec<File> = Vec::new();

                for line in output_lines {
                    let line_parts: Vec<&str> = line.split(' ').collect();

                    match line_parts[0] {
                        "dir" => continue, // Given the way I chose to model the filesystem, these are irrelevant
                        val if val.chars().all(|ch| char::is_ascii_digit(&ch)) => files.push((
                            line_parts[1].to_string(),
                            line_parts[0].parse::<u32>().unwrap(),
                        )),
                        _ => continue, // Shouldn't happen, let's ignore the instruction
                    }
                }

                directories.get_mut(&current_pwd).unwrap().1 = files;
            }
            _ => continue, // Shouldn't happen, let's ignore the instruction
        }
    }

    directories
}

fn get_directory_direct_size(files: Vec<File>) -> u32 {
    files.into_iter().fold(0, |acc, x| acc + x.1)
}

pub fn solve_a(input: String) -> Result<String, String> {
    let directories = build_directories(input);

    let res = directories
        .keys()
        .map(|directory_path| {
            // O(n^2) :scream:
            directories.iter().fold(0, |acc, (x_path, x_directory)| {
                acc + if x_path.starts_with(directory_path) {
                    get_directory_direct_size(x_directory.clone().1)
                } else {
                    0
                }
            })
        })
        .fold(0, |acc, x| acc + if x <= 100000 { x } else { 0 });

    Ok(res.to_string())
}

pub fn solve_b(input: String) -> Result<String, String> {
    let directories = build_directories(input);
    let mut directory_sizes: Vec<u32> = directories
        .keys()
        .map(|directory_path| {
            // O(n^2) :scream:
            directories.iter().fold(0, |acc, (x_path, x_directory)| {
                acc + if x_path.starts_with(directory_path) {
                    get_directory_direct_size(x_directory.clone().1)
                } else {
                    0
                }
            })
        })
        .collect();

    directory_sizes.sort();
    // "/" directory is the last element as it has the biggest size (contains all other directories)
    let space_used = directory_sizes[directory_sizes.len() - 1];
    let space_needed_to_free = 30_000_000 - (70_000_000 - space_used);

    for directory_size in directory_sizes {
        if directory_size > space_needed_to_free {
            return Ok(directory_size.to_string());
        }
    }

    Err("Solution not found!".to_string())
}
