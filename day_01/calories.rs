pub fn get_calories_per_elf(calorie_list: &str) -> Vec<i32> {
    let mut elf_calories = Vec::new();

    for calories in calorie_list.split("\n\n") {
        elf_calories.push(calories.lines().fold(0, |sum, food| sum + food.parse::<i32>().unwrap()));
    }

    return elf_calories;
}
