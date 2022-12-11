
fn get_starting_items(items: &str) -> Vec<u32> {
    let mut starting_items = items.split(": ");
    starting_items.next();
    let items = match starting_items.next() {
        Some(v) => v.split(',').map(|item| item.trim().parse::<u32>().unwrap()).collect(),
        None => Vec::new(),
    };

    return items;
}

fn add_by(old: u32, value: u32) -> u32 {
    old + value
}
fn multiply_by(old: u32, value: u32) -> u32 {
    old * value
}
fn multiply_by_itself(old: u32, _: u32) -> u32 {
    old * old
}

fn get_operation(operation_str: &str) -> (fn(u32, u32) -> u32, u32) {
    let mut operation = operation_str.split("new = old ");
    operation.next();
    let (op, value) = operation.next().unwrap().split_at(1);
    let op_value = match value.trim() {
        "old" => 0,
        v => v.parse::<u32>().unwrap(),
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

fn get_test(test_str: &str) -> u32 {
    let mut test = test_str.split("divisible by ");
    test.next();
    return test.next().unwrap().parse::<u32>().unwrap();
}

fn get_when_case(rest: &str) -> u32 {
    let mut true_false_case = rest.split("throw to monkey ");
    true_false_case.next();
    return true_false_case.next().unwrap().parse::<u32>().unwrap();
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<u32>,
    pub inspected: u32,
    pub op_fn: fn(u32, u32) -> u32,
    pub op_value: u32,
    pub test: u32,
    pub true_case: u32,
    pub false_case: u32,
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
            inspected: 0 as u32,
        };

        monkeys.push(monkey);
    }

    return monkeys;
}

// send_to, new_value
fn inspect_item(monke: &Monkey, divided_by: u32) -> Vec<(u32, u32)> {
    let mut send_to: Vec<(u32, u32)> = Vec::new();

    for item in monke.items.clone() {
        let new_value = (((monke.op_fn)(item, monke.op_value) / divided_by) as f64).floor() as u32;
        let tested = (new_value % monke.test) == 0;
        send_to.push((if tested { monke.true_case } else { monke.false_case }, new_value));
    }

    return send_to;
}

pub fn monkey_round(monkeys: &mut Vec<Monkey>, part_one: bool) {
    let divided_by = if part_one { 3 } else { 1 };
    for index in 0..monkeys.len() {
        let monke = &mut monkeys[index];
        let monkey_round = inspect_item(monke, divided_by);
        monke.inspected = monke.inspected + monkey_round.len() as u32;

        for (send_to, new_value) in monkey_round {
            monkeys[send_to as usize].items.push(new_value);
            monkeys[index].items = Vec::new();
        }
    }
}
