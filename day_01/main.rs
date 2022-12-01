mod calories;
use calories::get_calories_per_elf;
use calories::get_top_three_calories;

fn part_one() {
    println!("PART 1");
    println!("Test input");
    let calories_per_elf_test = get_calories_per_elf(include_str!("test_input.txt"));

    println!("The elf with more calories has {:?} calories", calories_per_elf_test.iter().max().unwrap());
    println!("-------------\n");

    println!("Real input");
    let calories_per_elf = get_calories_per_elf(include_str!("day_01_input.txt"));
    println!("The elf with more calories has {:?} calories", calories_per_elf.iter().max().unwrap());
    println!("-------------\n\n");
}

fn part_two() {
    println!("PART 2");
    println!("Test input");
    let calories_per_elf_test = get_calories_per_elf(include_str!("test_input.txt"));
    let top_three_calories_test = get_top_three_calories(calories_per_elf_test);

    println!("The three elves with the most calories have {:?} calories respectively", top_three_calories_test);
    println!("This adds up to {:?}", top_three_calories_test.iter().fold(0, |sum, calories| sum + calories));
    println!("-------------\n");

    println!("Real input");
    let calories_per_elf = get_calories_per_elf(include_str!("day_01_input.txt"));
    let top_three_calories = get_top_three_calories(calories_per_elf);

    println!("The three elves with the most calories have {:?} calories respectively", top_three_calories);
    println!("This adds up to {:?}", top_three_calories.iter().fold(0, |sum, calories| sum + calories));
    println!("-------------\n\n");
}

fn main() {
    part_one();

    part_two();
}
