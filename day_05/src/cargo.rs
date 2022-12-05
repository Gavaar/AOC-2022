use crate::files::CargoInstructions;

pub fn get_cargo_set(cargo_as_str: &str) -> Vec<Vec<char>> {
    let mut cargo_lines = cargo_as_str.lines().rev();
    let indexes: Vec<u16> = cargo_lines
        .next()
        .unwrap()
        .split("   ")
        .map(|i| i.trim().parse::<u16>().unwrap())
        .collect();
    let mut cargo: Vec<Vec<char>> = vec![Vec::new(); indexes.len()];

    for boxes in cargo_lines {
        for (box_index, single_box) in boxes.chars().enumerate() {
            if (box_index + 3) % 4 == 0 {
                if single_box != ' ' {
                    cargo[((box_index + 3) / 4) - 1].push(single_box)
                };
            }
        }
    }

    return cargo;
}

pub fn apply_instructions(mut file: CargoInstructions) -> CargoInstructions {
    for instruction in file.instructions.lines() {
        let instructions_parsed: Vec<usize> = instruction
            // .split_inclusive(char::is_numeric)
            .split_ascii_whitespace()
            .filter(|s| s.chars().all(char::is_numeric))
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        println!("{:?}", instructions_parsed);
        for _ in 0..(instructions_parsed[0]) {
            let popped_value = file.cargo[instructions_parsed[1] - 1].pop().unwrap();
            file.cargo[instructions_parsed[2] - 1].push(popped_value);
        }
    }

    return file;
}
