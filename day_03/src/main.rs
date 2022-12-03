mod utils;
use utils::test_file;
use utils::day_file;
use utils::parse_groups;
use utils::parse_rucksacks;

fn part_one(test: bool) {
    let rucksacks_raw = if test { test_file() } else { day_file() };
    let rucksacks = parse_rucksacks(rucksacks_raw);
    let rucksack_sum = rucksacks.iter().fold(0, |sum, ruck| sum as i32 + ruck.type_value as i32);

    println!("{}", if test { "----TEST---" } else { "----DAY----" });
    println!("{}", rucksack_sum);
}

fn part_two(test: bool) {
    let rucksacks = if test { test_file() } else { day_file() };
    let groups = parse_groups(rucksacks);
    let groups_sum: u32 = groups.iter().fold(0, |sum, elf| sum as u32 + *elf as u32);

    println!("{}", if test { "----TEST---" } else { "----DAY----" });
    println!("{:?}", groups_sum);
}

fn main() {
    println!("## PART 1 ##");
    part_one(true);
    part_one(false);

    println!("## PART 2 ##");
    part_two(true);
    part_two(false);
}
