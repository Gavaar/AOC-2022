mod files;
mod monkeys;

fn part_one_calculations(final_state_monke: Vec<monkeys::Monkey>) -> (u32, u32) {
    let mut inspected_list = final_state_monke.iter().map(|monke| monke.inspected).collect::<Vec<u32>>();
    inspected_list.sort();

    return (inspected_list.pop().unwrap(), inspected_list.pop().unwrap());
}

fn part_one(file: &'static str) {
    let mut monkeys = monkeys::parse_file(file);
    for _ in 0..20 {
        monkeys::monkey_round(&mut monkeys, true);
    }
    let (biggest, second_biggest) = part_one_calculations(monkeys);

    println!("biggest: {}, second: {}", biggest, second_biggest);
    println!("their multiplied values are: {}", biggest * second_biggest);
}

fn main() {
    let test_file = files::get(true);
    let day_file = files::get(false);

    println!("### PART 1 ###");
    println!("--> TEST\n");
    part_one(test_file);
    println!("--> DAY\n");
    part_one(day_file);

    println!("--------------");
    println!("### PART 2 ###");
}
