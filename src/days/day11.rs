struct Monkey {
    items: Vec<u64>,
    operation: (String, String, String),
    test: u64,
    true_destination: usize,
    false_destination: usize,
    items_inspected: u64,
}

impl Monkey {
    fn new(monkey_input: &str) -> Monkey {
        let parts: Vec<&str> = monkey_input.split('\n').collect();

        let items: Vec<u64> = parts[1].split(": ").collect::<Vec<&str>>()[1]
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let operation_parts: Vec<&str> = parts[2].split("= ").collect::<Vec<&str>>()[1]
            .split(' ')
            .collect();

        let test: u64 = parts[3].split("by ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        let true_destination: usize = parts[4].split("monkey ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        let false_destination: usize = parts[5].split("monkey ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        Monkey {
            items,
            operation: (
                operation_parts[0].to_string(),
                operation_parts[1].to_string(),
                operation_parts[2].to_string(),
            ),
            test,
            true_destination,
            false_destination,
            items_inspected: 0,
        }
    }

    fn take_turn(&mut self, divide_value: u64, modulo_value: u64) -> Vec<(usize, u64)> {
        let mut thrown_items: Vec<(usize, u64)> = Vec::new();

        // Process items
        for item in &self.items {
            let operand_1 = match self.operation.0.as_str() {
                "old" => *item,
                number => number.parse::<u64>().unwrap(),
            };
            let operand_2 = match self.operation.2.as_str() {
                "old" => *item,
                number => number.parse::<u64>().unwrap(),
            };

            let mut new_value = match self.operation.1.as_str() {
                "*" => operand_1 * operand_2,
                "+" => operand_1 + operand_2,
                _ => 0, // Shouldn't happen
            } / divide_value;

            // Only needed for part two, using 0 as "do not do this"
            if modulo_value != 0 {
                new_value %= modulo_value;
            }

            let thrown_to = if new_value % self.test == 0 {
                self.true_destination
            } else {
                self.false_destination
            };
            thrown_items.push((thrown_to, new_value));

            self.items_inspected += 1;
        }

        // All items are thrown - so reset the items
        self.items = Vec::new();

        thrown_items
    }

    fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }
}

pub fn solve_a(input: String) -> Result<String, String> {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::new).collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let thrown_items = monkeys[i].take_turn(3, 0);

            for (thrown_to, item) in thrown_items {
                monkeys[thrown_to].add_item(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));
    let res = monkeys[0].items_inspected * monkeys[1].items_inspected;

    Ok(res.to_string())
}

pub fn solve_b(input: String) -> Result<String, String> {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::new).collect();

    // Given that the only relevant operation that we do with the items values is modulo "test"
    // to a different value for each monkey, we don't need to keep the whole value of each item,
    // just the result of the modulo of the items with a common multiple, as this ensures that
    // the modulos to each value are always correct (see Chinese Remainder Theorem)
    // Multiplying all the values is the easiest way to obtain a common multiple :D
    let common_multiple = monkeys.iter().fold(1, |mult, x| mult * x.test);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let thrown_items = monkeys[i].take_turn(1, common_multiple);

            for (thrown_to, item) in thrown_items {
                monkeys[thrown_to].add_item(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));
    let res = monkeys[0].items_inspected * monkeys[1].items_inspected;

    Ok(res.to_string())
}
