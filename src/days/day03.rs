use std::collections::HashSet;

fn get_repeated_item(items: &str) -> char {
    let half = items.len() / 2;
    let mut seen = HashSet::new();

    for (index, item) in items.chars().enumerate() {
        if index < half {
            seen.insert(item);
        } else if seen.contains(&item) {
            return item;
        }
    }

    // Shouldn't happen (problem states that there are always repeated items, didn't bother to wrap in Result)
    ' '
}

fn get_priority(item: char) -> u32 {
    if item.is_lowercase() {
        item as u32 - 96 // 97 is ASCII Code for 'a', +1 because 'a' has value 1 not 0
    } else {
        item as u32 - 38 // 65 is ASCII Code for 'A', +27 because 'A' has value 27
    }
}

pub fn solve_a(input: String) -> Result<String, String> {
    let res: u32 = input
        .split("\n")
        .map(|x| get_priority(get_repeated_item(x)))
        .sum();

    Ok(res.to_string())
}

fn get_common_item_in_group(group: [&str; 3]) -> char {
    let mut seen = HashSet::new();

    for i in 0..group.len() {
        if i == 0 {
            for item in group[i].chars() {
                seen.insert(item);
            }
        } else {
            let mut new_seen = HashSet::new();

            for item in group[i].chars() {
                if seen.contains(&item) {
                    new_seen.insert(item);
                }
            }

            seen = new_seen;
        }
    }

    *seen.iter().next().unwrap() // According to the problem statement, should only have one common item
}

pub fn solve_b(input: String) -> Result<String, String> {
    let res: u32 = input
        .split("\n")
        .into_iter()
        .array_chunks::<3>()
        .map(|x| get_priority(get_common_item_in_group(x)))
        .sum();

    Ok(res.to_string())
}
