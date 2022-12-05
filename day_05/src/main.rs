mod cargo;
use cargo::apply_instructions;

mod files;
use files::test_file;
use files::day_file;

fn part_one(test: bool, crate_moover_9001: bool) {
    let mut file = if test {test_file()} else {day_file()};
    file = apply_instructions(file, crate_moover_9001);

    let mut final_str = String::from("");

    for stack in file.cargo {
        final_str.push(*stack.last().unwrap());
    }

    println!("The final string is: {}", final_str);
}

fn main() {
    println!("## PART 1 ##");
    part_one(true, false);
    part_one(false, false);

    println!("## PART 2 ##");
    part_one(true, true);
    part_one(false, true);
}
