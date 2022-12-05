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

fn parse_instructions(instruction: &str) -> Vec<usize> {
    return instruction
        .split_ascii_whitespace()
        .filter(|s| s.chars().all(char::is_numeric))
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
}

pub fn apply_instructions(mut file: CargoInstructions, crate_moover_9001: bool) -> CargoInstructions {
    for instruction in file.instructions.lines() {
        let instructions_parsed: Vec<usize> = parse_instructions(instruction);
        let mut moved_objects = Vec::new();

        for _ in 0..(instructions_parsed[0]) {
            let popped_value = file.cargo[instructions_parsed[1] - 1].pop().unwrap();
            moved_objects.push(popped_value);
        }

        if crate_moover_9001 { moved_objects.reverse() }

        for i in 0..(moved_objects.len()) {
            file.cargo[instructions_parsed[2] - 1].push(moved_objects[i]);
        }
    }

    return file;
}
