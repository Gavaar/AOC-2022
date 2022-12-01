mod calories;
use calories::get_calories_per_elf;
use calories::get_top_three_calories;

fn part_one() {
    println!("Test input");
    let calories_per_elf_test = get_calories_per_elf(include_str!("inputs/test_input.txt"));

    println!("The elf with more calories has {:?} calories", calories_per_elf_test.iter().max().unwrap());
    println!("-------------");

    println!("Real input");
    let calories_per_elf = get_calories_per_elf(include_str!("inputs/day_01_input.txt"));
    println!("The elf with more calories has {:?} calories", calories_per_elf.iter().max().unwrap());
}

fn part_two() {
    println!("Test input");
    let calories_per_elf_test = get_calories_per_elf(include_str!("inputs/test_input.txt"));
    let top_three_calories_test = get_top_three_calories(calories_per_elf_test);

    println!("The three elves with the most calories have {:?} calories respectively", top_three_calories_test);
    println!("This adds up to {:?}", top_three_calories_test.iter().fold(0, |sum, calories| sum + calories));
    println!("-------------");

    println!("Real input");
    let calories_per_elf = get_calories_per_elf(include_str!("inputs/day_01_input.txt"));
    let top_three_calories = get_top_three_calories(calories_per_elf);

    println!("The three elves with the most calories have {:?} calories respectively", top_three_calories);
    println!("This adds up to {:?}", top_three_calories.iter().fold(0, |sum, calories| sum + calories));
}

fn main() {
    println!("## PART 1");
    part_one();
    println!("-------------\n\n");

    println!("## PART 2");
    part_two();
    println!("-------------\n\n");
}
