use std::cmp::max;
use std::collections::HashSet;

// Not proud of the code, too much repeated code :)
// But was fast enough to solve and didn't bother to refactor

pub fn solve_a(input: String) -> Result<String, String> {
    let matrix: Vec<Vec<i32>> = input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let num_lines = matrix.len();
    let line_length = matrix[0].len();

    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    for (i, line) in matrix.iter().enumerate() {
        let mut highest = -1;

        // Iterate left to right
        for (j, tree) in line.iter().enumerate() {
            if *tree > highest {
                visible.insert((i, j));
                highest = *tree;
            }
        }

        highest = -1;

        // Iterate right to left
        for (j, tree) in line.iter().rev().enumerate() {
            if *tree > highest {
                visible.insert((i, line_length - j - 1));
                highest = *tree;
            }
        }
    }

    for j in 0..line_length {
        let mut highest = -1;

        // Iterate top to bottom
        for (i, line) in matrix.iter().enumerate().take(num_lines) {
            let tree = line[j];
            if tree > highest {
                visible.insert((i, j));
                highest = tree;
            }
        }

        highest = -1;

        // Iterate bottom to top
        for i in (0..line_length).rev() {
            let tree = matrix[i][j];
            if tree > highest {
                visible.insert((i, j));
                highest = tree;
            }
        }
    }

    let res = visible.len();

    Ok(res.to_string())
}

pub fn solve_b(input: String) -> Result<String, String> {
    let matrix: Vec<Vec<i32>> = input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let num_lines = matrix.len();
    let line_length = matrix[0].len();

    let mut res = 0;

    for (candidate_i, candidate_line) in matrix.iter().enumerate() {
        // We can ignore the edges as they will always have score 0
        // Makes the inner loops easier as they don't have to check for out of bounds
        if candidate_i == 0 || candidate_i == num_lines - 1 {
            continue;
        }

        for (candidate_j, candidate_tree) in candidate_line.iter().enumerate() {
            if candidate_j == 0 || candidate_j == line_length - 1 {
                continue;
            }

            let mut score = 1;
            let mut visible = 0;

            // Check right
            for tree in &candidate_line[candidate_j + 1..] {
                visible += 1;

                if *tree >= *candidate_tree {
                    break;
                }
            }

            score *= visible;
            visible = 0;

            // Iterate left
            for tree in candidate_line[..candidate_j].iter().rev() {
                visible += 1;

                if *tree >= *candidate_tree {
                    break;
                }
            }

            score *= visible;
            visible = 0;

            // Check down
            for line in matrix.iter().take(num_lines).skip(candidate_i + 1) {
                let tree = line[candidate_j];

                visible += 1;
                if tree >= *candidate_tree {
                    break;
                }
            }

            score *= visible;
            visible = 0;

            // Check up
            for i in (0..candidate_i).rev() {
                let tree = matrix[i][candidate_j];

                visible += 1;
                if tree >= *candidate_tree {
                    break;
                }
            }

            score *= visible;
            res = max(score, res);
        }
    }

    Ok(res.to_string())
}
