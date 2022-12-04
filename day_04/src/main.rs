mod files;
use files::test_file;
use files::day_file;

mod utils;
use utils::contained_by_partner;

fn part_one(test: bool) -> i32 {
    let test_file = if test { test_file() } else { day_file() };
    let contained_list = contained_by_partner(test_file);
    let contained_amount = contained_list.iter().fold(0, |sum, contained| sum + if *contained { 1 } else { 0 });
    return contained_amount;
}

fn main() {
    println!("## PART 1 ##");
    let mut contained_size = part_one(true);
    println!("There were {} redundant groups", contained_size);

    contained_size = part_one(false);
    println!("There were {} redundant groups", contained_size);
}
