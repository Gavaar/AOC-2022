mod files;
use files::day_file;
use files::test_file;

use std::collections::HashMap;

fn part_one(file: &HashMap<String, u32>) -> u32 {
    let mut sum_of_big_dirs: u32 = 0;

    for (name, size) in file {
        if name.contains("/__") {
            continue;
        }

        if *size <= 100_000 {
            sum_of_big_dirs += size;
        }
    }

    return sum_of_big_dirs;
}

fn part_two(file: &HashMap<String, u32>) -> u32 {
    let disk_space: u32 = 70_000_000;
    let update_space_needed: u32 = 30_000_000;

    let used_space = file.iter().map(|(_, v)| v).max().unwrap();
    let needed_space = update_space_needed - (disk_space - used_space);

    let deletable_files = file.iter().filter(|(name, size)| {
        return !name.contains("/__") && **size >= needed_space;
    }).map(|(_, size)| *size);

    println!("{:?}", &deletable_files.clone().collect::<Vec<u32>>());

    let smallest_deletable = deletable_files.min().unwrap();

    return smallest_deletable;
}

fn main() {
    let file_map_test = test_file();
    let file_map = day_file();

    println!("## PART 1 ##");
    println!("Test: the sum is {}", part_one(&file_map_test));
    println!("Input: the sum is {}", part_one(&file_map));

    println!("## PART 2 ##");
    println!("Test: the smallest to delete is {}", part_two(&file_map_test));
    println!("Input: the smallest to delete is {}", part_two(&file_map));
}
