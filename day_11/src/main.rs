mod files;
mod monkeys;

fn worry_calculator(final_state_monke: Vec<monkeys::Monkey>) -> (u128, u128) {
    let mut inspected_list = final_state_monke.iter().map(|monke| monke.inspected).collect::<Vec<u128>>();
    inspected_list.sort();

    return (inspected_list.pop().unwrap(), inspected_list.pop().unwrap());
}

fn part_one(file: &'static str) {
    let mut monkeys = monkeys::parse_file(file);

    for _ in 0..20 {
        monkeys::monkey_round(&mut monkeys, true);
    }

    let (biggest, second_biggest) = worry_calculator(monkeys);

    println!("biggest: {}, second: {}", biggest, second_biggest);
    println!("their multiplied values are: {}", biggest * second_biggest);
}

fn part_two(file: &'static str) {
    let mut monkeys = monkeys::parse_file(file);

    for _ in 0..10000 {
        monkeys::monkey_round(&mut monkeys, false);
    }

    let (biggest, second_biggest) = worry_calculator(monkeys);

    println!("biggest: {}, second: {}", biggest, second_biggest);
    println!("their multiplied values are: {}", biggest * second_biggest);
}

fn main() {
    let test_file = files::get(true);
    let day_file = files::get(false);

    println!("### PART 1 ###");
    println!("--> TEST");
    part_one(test_file);
    println!("--> DAY");
    part_one(day_file);

    println!("--------------");
    println!("### PART 2 ###");
    println!("--> TEST\n");
    part_two(test_file);
    println!("--> DAY\n");
    part_two(day_file);
}
