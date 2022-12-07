mod files;
use files::test_file;
use files::day_file;

use std::collections::HashMap;

fn part_one(file: HashMap<String, u32>) -> u32 {
    let mut sum_of_big_dirs: u32 = 0;

    for (name, size) in file {
        if name.contains("/__") {
            continue;
        }

        if size <= 100000 {
            sum_of_big_dirs += size;
        }
    }

    return sum_of_big_dirs;
}

fn main() {
    println!("## PART 1 ##");
    let file_map_test = test_file();
    let file_map = day_file();

    println!("Test: the sum is {}", part_one(file_map_test));
    println!("Input: the sum is {}", part_one(file_map));
}
