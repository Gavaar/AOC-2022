mod files;
mod trees;

use std::collections::HashMap;

fn sum_of_trees(visible_trees: HashMap<String, bool>) -> u16 {
    return visible_trees
        .values()
        .fold(0, |acc, v| if *v { acc + 1 } else { acc });
}

fn part_one(file: &'static str) {
    let parsed_grid = files::parse_file(file);
    let (visible_trees, _) = trees::are_trees_visible(parsed_grid);
    let sum = sum_of_trees(visible_trees);

    println!("Trees added up are {}", sum);
}

fn part_two(file: &'static str) {
    let parsed_grid = files::parse_file(file);
    let (_, tree_scores) = trees::are_trees_visible(parsed_grid);
    let biggest = tree_scores.values().max().unwrap();

    println!("Biggest tree score: {}", biggest);
}

fn main() {
    let test_file = files::file(true);
    let day_file = files::file(false);

    println!("## PART 1 ##");
    part_one(test_file);
    part_one(day_file);
    println!("------------");
    println!("## PART 2 ##");
    part_two(test_file);
    part_two(day_file);
}
