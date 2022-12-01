pub fn get_calories_per_elf(calorie_list: &str) -> Vec<i32> {
    let mut elf_calories = Vec::new();

    for calories in calorie_list.split("\n\n") {
        elf_calories.push(calories.lines().fold(0, |sum, food| sum + food.parse::<i32>().unwrap()));
    }

    return elf_calories;
}

pub fn get_top_three_calories(calorie_list: Vec<i32>) -> Vec<i32> {
    let mut top_three = vec![0; 5];

    for calories in calorie_list.iter() {
        for index in 0..3 {
            if calories > &top_three[index] {
                top_three.insert(index, *calories);
                top_three.resize(3, 0);
                break;
            };
        }
    }

    return top_three;
}
