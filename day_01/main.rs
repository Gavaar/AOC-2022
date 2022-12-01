mod calories;
use calories::get_calories_per_elf;

fn main() {
    println!("Test input");
    let calories_per_elf_test = get_calories_per_elf(include_str!("test_input.txt"));

    println!("The elf with more calories has {:?} calories", calories_per_elf_test.iter().max().unwrap());
    println!("-------------");

    println!("Real input part #1");
    let calories_per_elf = get_calories_per_elf(include_str!("day_01_input.txt"));
    println!("The elf with more calories has {:?} calories", calories_per_elf.iter().max().unwrap());
}
