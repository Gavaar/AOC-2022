mod cargo;
use cargo::apply_instructions;

mod files;
use files::test_file;

use crate::files::day_file;
// use files::day_file;

fn part_one(test: bool) {
    let mut file = if test {test_file()} else {day_file()};
    file = apply_instructions(file);

    let mut final_str = String::from("");

    for stack in file.cargo {
        final_str.push(*stack.last().unwrap());
    }

    println!("The final string is: {}", final_str);
}

fn main() {
    println!("## PART 1 ##");
    part_one(true);
    part_one(false);
}
