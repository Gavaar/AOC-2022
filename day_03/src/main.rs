mod files;
use files::test_file;
use files::day_file;

fn part_one(test: bool) {
    let rucksacks = if test { test_file() } else { day_file() };
    let rucksack_sum = rucksacks.iter().fold(0, |sum, ruck| sum as i32 + ruck.type_value as i32);

    println!("{}", if test { "----TEST---" } else { "----DAY----" });
    println!("{}", rucksack_sum);
}

// fn part_two(test: bool) {
//     let rucksacks = if test { test_file() } else { day_file() };

//     println!("{}", if test { "----TEST---" } else { "----DAY----" });
//     println!("{}", rucksack_sum);
// }

fn main() {
    println!("## PART 1 ##");
    part_one(true);
    part_one(false);

    // println!("## PART 2 ##");
    // part_two(true);
    // part_two(false);
}
