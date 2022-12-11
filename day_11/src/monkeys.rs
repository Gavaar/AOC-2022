
fn get_starting_items(items: &str) -> Vec<u128> {
    let mut starting_items = items.split(": ");
    starting_items.next();
    let items = match starting_items.next() {
        Some(v) => v.split(',').map(|item| item.trim().parse::<u128>().unwrap()).collect(),
        None => Vec::new(),
    };

    return items;
}

fn add_by(old: u128, value: u128) -> u128 {
    old + value
}
fn multiply_by(old: u128, value: u128) -> u128 {
    old * value
}
fn multiply_by_itself(old: u128, _: u128) -> u128 {
    old * old
}

fn get_operation(operation_str: &str) -> (fn(u128, u128) -> u128, u128) {
    let mut operation = operation_str.split("new = old ");
    operation.next();
    let (op, value) = operation.next().unwrap().split_at(1);
    let op_value = match value.trim() {
        "old" => 0,
        v => v.parse::<u128>().unwrap(),
    };
    let op_fn = if op_value == 0 {
        multiply_by_itself
    } else {
        match op {
            "+" => add_by,
            "*" => multiply_by,
            _ => panic!("Monkey operation was not sum or multiply"),
        }
    };
    return (op_fn, op_value);
}

fn get_test(test_str: &str) -> u128 {
    let mut test = test_str.split("divisible by ");
    test.next();
    return test.next().unwrap().parse::<u128>().unwrap();
}

fn get_when_case(rest: &str) -> u128 {
    let mut true_false_case = rest.split("throw to monkey ");
    true_false_case.next();
    return true_false_case.next().unwrap().parse::<u128>().unwrap();
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<u128>,
    pub inspected: u128,
    pub op_fn: fn(u128, u128) -> u128,
    pub op_value: u128,
    pub test: u128,
    pub true_case: u128,
    pub false_case: u128,
}

pub fn parse_file(file: &'static str) -> Vec<Monkey> {
    let monkeys_str = file.split("\n\n");
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey in monkeys_str {
        let mut monkey_ops = monkey.lines();
        monkey_ops.next();
        let starting_items = get_starting_items(monkey_ops.next().unwrap());
        let (op_fn, op_value) = get_operation(monkey_ops.next().unwrap());
        let test = get_test(monkey_ops.next().unwrap());
        let true_case = get_when_case(monkey_ops.next().unwrap());
        let false_case = get_when_case(monkey_ops.next().unwrap());
        let monkey = Monkey {
            op_fn,
            op_value,
            test,
            true_case,
            false_case,
            items: starting_items,
            inspected: 0 as u128,
        };

        monkeys.push(monkey);
    }

    return monkeys;
}

// send_to, new_value
fn inspect_item(monke: &Monkey, divided_by: u128, mincom: u128) -> Vec<(u128, u128)> {
    let mut send_to: Vec<(u128, u128)> = Vec::new();

    for item in monke.items.clone() {
        let new_value = (((monke.op_fn)(item, monke.op_value) / divided_by) as f64).floor() as u128;
        let tested = (new_value % monke.test) == 0;

        let target = if tested { monke.true_case } else { monke.false_case };
        let value_to_push: u128 = new_value % mincom;

        send_to.push((target, value_to_push));
    }

    return send_to;
}

pub fn monkey_round(monkeys: &mut Vec<Monkey>, part_one: bool) {
    let min_comm_mul = monkeys.iter().fold(1, |acc, m| m.test * acc);

    let divided_by = if part_one { 3 } else { 1 };
    for index in 0..monkeys.len() {
        let monke = &mut monkeys[index];
        let monkey_round = inspect_item(monke, divided_by, min_comm_mul);
        monke.inspected = monke.inspected + monkey_round.len() as u128;

        for (send_to, new_value) in monkey_round {
            monkeys[send_to as usize].items.push(new_value);
            monkeys[index].items = Vec::new();
        }
    }
}
