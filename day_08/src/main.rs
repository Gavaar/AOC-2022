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
    let visible_trees = trees::are_trees_visible(parsed_grid);
    let sum = sum_of_trees(visible_trees);

    println!("Trees added up are {}", sum);
}

fn main() {
    println!("## PART 1 ##");
    part_one(files::file(true));
    part_one(files::file(false));
}
