use std::collections::HashMap;

#[derive(Debug)]
pub struct RuckSack {
    // I thought I was gonna use this
    // first_compartment: &'a str,
    // second_compartment: &'a str,
    // common_type: char,
    pub type_value: u8,
}

fn char_value(char: &char) -> u8 {
    if char.is_uppercase() {
        // uppercases go from 27 - 52, so I substract 38 from uppercase char value
        return *char as u8 - 38;
    }

    return *char as u8 - 96;
}

fn compartment_into_hashmap(compartment: &str) -> HashMap<char, bool> {
    let mut hash_map: HashMap<char, bool> = HashMap::new();

    for char in compartment.chars() {
        hash_map.insert(char, true);
    }

    return hash_map;
}

pub fn parse_rucksacks<>(rucksacks: Vec<&str>)-> Vec<RuckSack> {
    let mut parsed_rucksacks: Vec<RuckSack> = Vec::new();

    for rucksack in rucksacks {
        let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);
        let first_compartment_hasmap = compartment_into_hashmap(first_compartment);
        let common_letter = second_compartment.chars()
            .find(|char| *first_compartment_hasmap.get(char).unwrap_or(&false))
            .unwrap();
        let common_letter_value = char_value(&common_letter);

        parsed_rucksacks.push(RuckSack {
            // first_compartment,
            // second_compartment,
            // common_type: common_letter.clone(),
            type_value: common_letter_value,
        });
    }

    return parsed_rucksacks;
}

pub fn parse_groups(rucksacks: Vec<&str>) -> Vec<u8> {
    let mut group_values: Vec<u8> = Vec::new();

    for index in 0..(rucksacks.len() / 3) {
        let (group, _) = rucksacks.split_at((index + 1) * 3);
        let elfs: Vec<&&str> = group.iter().rev().take(3).rev().collect();

        let first_elf = compartment_into_hashmap(elfs[0]);
        let second_elf = compartment_into_hashmap(elfs[1]);
        let common_char = elfs[2].chars()
            .find(|char| *first_elf.get(char).unwrap_or(&false) && *second_elf.get(char).unwrap_or(&false))
            .unwrap();

        group_values.push(char_value(&common_char));
    }

    return group_values;
}

pub fn test_file<'a>() -> Vec<&'a str> {
    return include_str!("./inputs/test.txt").split("\n").collect();
}

pub fn day_file<'a>() ->Vec<&'a str> {
    return include_str!("./inputs/day.txt").split("\n").collect();
}
