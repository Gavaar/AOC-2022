mod files;
use files::test_file;
use files::day_file;

mod utils;
use utils::contained_by_partner;

fn part_one(file: &Vec<Vec<&'static str>>) -> i32 {
    let contained_list = contained_by_partner(file, false);
    let contained_amount = contained_list.iter().fold(0, |sum, contained| sum + if *contained { 1 } else { 0 });
    return contained_amount;
}

fn part_two(file: &Vec<Vec<&'static str>>) -> i32 {
    let overlap_list = contained_by_partner(file, true);
    let contained_amount = overlap_list.iter().fold(0, |sum, overlaps| sum + if *overlaps { 1 } else { 0 });
    return contained_amount;
}

fn main() {
    let test_file = test_file();
    let day_file = day_file();

    println!("## PART 1 ##");
    let mut contained_size = part_one(&test_file);
    let mut overlapping_size = part_two(&test_file);
    println!("There were {} redundant groups", contained_size);
    println!("There were {} overlapping groups", overlapping_size);

    println!("## PART 2 ##");
    contained_size = part_one(&day_file);
    overlapping_size = part_two(&day_file);
    println!("There were {} redundant groups", contained_size);
    println!("There were {} overlapping groups", overlapping_size);
}
