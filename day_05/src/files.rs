use super::cargo::get_cargo_set;

pub struct CargoInstructions {
    pub instructions: &'static str,
    pub cargo: Vec<Vec<char>>,
}

fn get_file(test: bool) -> CargoInstructions {
    let base_file: &'static str = if test {
        include_str!("./inputs/test.txt")
    } else {
        include_str!("./inputs/day.txt")
    };

    let (cargo_str, instructions) = base_file.split_at(base_file.find("\n\n").unwrap());
    let cargo = get_cargo_set(cargo_str);

    return CargoInstructions {
        cargo,
        instructions: instructions.trim(),
    }
}

pub fn test_file() -> CargoInstructions {
    return get_file(true);
}

pub fn day_file() -> CargoInstructions {
    return get_file(false);
}
