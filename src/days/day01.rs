pub fn solve_a(input: String) -> Result<String, String> {
    let res = input
        .split("\n\n")
        .map(|x| {
            x.split("\n").fold(0, |acc, x| {
                acc + match x.parse::<u32>() {
                    Ok(val) => val,
                    Err(_) => 0,
                }
            })
        })
        .max()
        .unwrap();

    Ok(res.to_string())
}

pub fn solve_b(input: String) -> Result<String, String> {
    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|x| {
            x.split("\n").fold(0, |acc, x| {
                acc + match x.parse::<u32>() {
                    Ok(val) => val,
                    Err(_) => 0,
                }
            })
        })
        .collect();
    elves.sort_by(|a, b| b.cmp(a));

    let res: u32 = elves[0..3].into_iter().sum();

    Ok(res.to_string())
}
