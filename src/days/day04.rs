fn get_pairs(line: &str) -> ((u32, u32), (u32, u32)) {
    let pairs: Vec<&str> = line.split(',').collect();
    let first_assigment: Vec<&str> = pairs[0].split('-').collect();
    let second_assigment: Vec<&str> = pairs[1].split('-').collect();

    (
        (
            first_assigment[0].parse().unwrap(),
            first_assigment[1].parse().unwrap(),
        ),
        (
            second_assigment[0].parse().unwrap(),
            second_assigment[1].parse().unwrap(),
        ),
    )
}

// Returns 1 if one of the pairs is fully contained on the other, 0 otherwise
fn is_fully_contained(pair: ((u32, u32), (u32, u32))) -> u32 {
    if (pair.0 .0 <= pair.1 .0 && pair.0 .1 >= pair.1 .1)
        || (pair.0 .0 >= pair.1 .0 && pair.0 .1 <= pair.1 .1)
    {
        1
    } else {
        0
    }
}

// Returns 1 if the pairs are overlapping, 0 otherwise
fn is_overlapping(pair: ((u32, u32), (u32, u32))) -> u32 {
    if pair.0 .1 < pair.1 .0 || pair.0 .0 > pair.1 .1 {
        0
    } else {
        1
    }
}

pub fn solve_a(input: String) -> Result<String, String> {
    let res: u32 = input
        .split('\n')
        .map(|x| is_fully_contained(get_pairs(x)))
        .sum();

    Ok(res.to_string())
}

pub fn solve_b(input: String) -> Result<String, String> {
    let res: u32 = input
        .split('\n')
        .map(|x| is_overlapping(get_pairs(x)))
        .sum();

    Ok(res.to_string())
}
