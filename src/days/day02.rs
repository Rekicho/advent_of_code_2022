use std::collections::HashMap;

thread_local! {
static SHAPES_TO_SCORE: HashMap<(char, char), u32> = [
    (('A', 'X'), 4),
    (('A', 'Y'), 8),
    (('A', 'Z'), 3),
    (('B', 'X'), 1),
    (('B', 'Y'), 5),
    (('B', 'Z'), 9),
    (('C', 'X'), 7),
    (('C', 'Y'), 2),
    (('C', 'Z'), 6),
].iter().cloned().collect();

static SHAPE_RESULT_TO_SCORE: HashMap<(char, char), u32> = [
    (('A', 'X'), 3),
    (('A', 'Y'), 4),
    (('A', 'Z'), 8),
    (('B', 'X'), 1),
    (('B', 'Y'), 5),
    (('B', 'Z'), 9),
    (('C', 'X'), 2),
    (('C', 'Y'), 6),
    (('C', 'Z'), 7),
].iter().cloned().collect();
}

fn get_score(shape_a: char, shape_b: char, part: char) -> u32 {
    match part {
        'a' => SHAPES_TO_SCORE.with(|map| map.get(&(shape_a, shape_b)).copied().unwrap_or(0)),
        'b' => SHAPE_RESULT_TO_SCORE.with(|map| map.get(&(shape_a, shape_b)).copied().unwrap_or(0)),
        _ => 0, // Shouldn't happen
    }
}

pub fn solve_a(input: String) -> Result<String, String> {
    let res: u32 = input
        .split('\n')
        .map(|x| get_score(x.chars().next().unwrap(), x.chars().nth(2).unwrap(), 'a'))
        .sum();

    Ok(res.to_string())
}

pub fn solve_b(input: String) -> Result<String, String> {
    let res: u32 = input
        .split('\n')
        .map(|x| get_score(x.chars().next().unwrap(), x.chars().nth(2).unwrap(), 'b'))
        .sum();

    Ok(res.to_string())
}
