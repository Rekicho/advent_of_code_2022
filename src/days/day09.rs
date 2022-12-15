use std::collections::HashSet;

type Position = (i32, i32);

pub fn solve_a(input: String) -> Result<String, String> {
    let mut current_head = (0, 0);
    let mut current_tail = (0, 0);
    let mut visited_tail: HashSet<Position> = HashSet::new();

    // Add initial tail position to visited
    visited_tail.insert(current_tail);

    for instruction in input.split('\n') {
        let parts: Vec<&str> = instruction.split(' ').collect();
        let direction = parts[0];
        let distance: i32 = parts[1].parse().unwrap();

        for _ in 0..distance {
            // Move head
            let movement = match direction {
                "U" => (0, 1),
                "R" => (1, 0),
                "D" => (0, -1),
                "L" => (-1, 0),
                _ => (0, 0), // Shouldn't happen according to problem definition
            };

            current_head = (current_head.0 + movement.0, current_head.1 + movement.1);

            // If necessary -> move tail
            let delta = (
                current_head.0 - current_tail.0,
                current_head.1 - current_tail.1,
            );

            // As the head and tail move 1 unit at a time, and the tails always follows the head
            // (with worst case 1 unit of time of delay) we know that the delta in a given direction
            // is always [-2; 2]. We only need to move the tail if, in at least one direction, the |delta| is 2
            // Also the |delta| can't be 2 in both directions, as the head can't move diagonally
            match delta {
                // Straight Up
                (2, 0) => current_tail = (current_tail.0 + 1, current_tail.1),

                // Up and Right
                (2, 1) => current_tail = (current_tail.0 + 1, current_tail.1 + 1),
                (1, 2) => current_tail = (current_tail.0 + 1, current_tail.1 + 1),

                // Straight Right
                (0, 2) => current_tail = (current_tail.0, current_tail.1 + 1),

                // Right and Down
                (-1, 2) => current_tail = (current_tail.0 - 1, current_tail.1 + 1),
                (-2, 1) => current_tail = (current_tail.0 - 1, current_tail.1 + 1),

                // Straight Down
                (-2, 0) => current_tail = (current_tail.0 - 1, current_tail.1),

                // Down and Left
                (-2, -1) => current_tail = (current_tail.0 - 1, current_tail.1 - 1),
                (-1, -2) => current_tail = (current_tail.0 - 1, current_tail.1 - 1),

                // Straight Left
                (0, -2) => current_tail = (current_tail.0, current_tail.1 - 1),

                // Left and Up
                (1, -2) => current_tail = (current_tail.0 + 1, current_tail.1 - 1),
                (2, -1) => current_tail = (current_tail.0 + 1, current_tail.1 - 1),

                // For all other cases - no need to move the tail
                _ => (),
            }

            // Add tail position to visited
            visited_tail.insert(current_tail);
        }
    }

    let res = visited_tail.len();

    Ok(res.to_string())
}

pub fn solve_b(input: String) -> Result<String, String> {
    let mut knots = [(0, 0); 10];
    let mut visited_tail: HashSet<Position> = HashSet::new();

    // Add initial tail position to visited
    visited_tail.insert((0, 0));

    for instruction in input.split('\n') {
        let parts: Vec<&str> = instruction.split(' ').collect();
        let direction = parts[0];
        let distance: i32 = parts[1].parse().unwrap();

        for _ in 0..distance {
            // As we are mutably borrowing the knots array, we can't access the previous knot in the for loop
            // So we keep track of the position of the previous knot with this
            let mut previous_knot = (0, 0);

            for (i, knot) in knots.iter_mut().enumerate() {
                if i == 0 {
                    // Move head
                    let movement = match direction {
                        "U" => (0, 1),
                        "R" => (1, 0),
                        "D" => (0, -1),
                        "L" => (-1, 0),
                        _ => (0, 0), // Shouldn't happen according to problem definition
                    };

                    *knot = (knot.0 + movement.0, knot.1 + movement.1);
                    previous_knot = (knot.0, knot.1);
                } else {
                    // Move tails
                    let delta = (previous_knot.0 - knot.0, previous_knot.1 - knot.1);

                    // As, different from the head, the previous tail may move diagonally, we need to consider the cases
                    // were |delta| is 2 in both directions
                    match delta {
                        // Straight Up
                        (2, 0) => *knot = (knot.0 + 1, knot.1),

                        // Up and Right
                        (2, 1) => *knot = (knot.0 + 1, knot.1 + 1),
                        (2, 2) => *knot = (knot.0 + 1, knot.1 + 1),
                        (1, 2) => *knot = (knot.0 + 1, knot.1 + 1),

                        // Straight Right
                        (0, 2) => *knot = (knot.0, knot.1 + 1),

                        // Right and Down
                        (-1, 2) => *knot = (knot.0 - 1, knot.1 + 1),
                        (-2, 2) => *knot = (knot.0 - 1, knot.1 + 1),
                        (-2, 1) => *knot = (knot.0 - 1, knot.1 + 1),

                        // Straight Down
                        (-2, 0) => *knot = (knot.0 - 1, knot.1),

                        // Down and Left
                        (-2, -1) => *knot = (knot.0 - 1, knot.1 - 1),
                        (-2, -2) => *knot = (knot.0 - 1, knot.1 - 1),
                        (-1, -2) => *knot = (knot.0 - 1, knot.1 - 1),

                        // Straight Left
                        (0, -2) => *knot = (knot.0, knot.1 - 1),

                        // Left and Up
                        (1, -2) => *knot = (knot.0 + 1, knot.1 - 1),
                        (2, -2) => *knot = (knot.0 + 1, knot.1 - 1),
                        (2, -1) => *knot = (knot.0 + 1, knot.1 - 1),

                        // For all other cases - no need to move the tail
                        _ => (),
                    }

                    previous_knot = (knot.0, knot.1);
                }
            }

            // Tail is the last knot
            visited_tail.insert(knots[9]);
        }
    }

    let res = visited_tail.len();

    Ok(res.to_string())
}
