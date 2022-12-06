fn get_file(number: u8) -> &'static str {
    return match number {
        0 => include_str!("./inputs/day.txt"),
        1 => include_str!("./inputs/test_1.txt"),
        2 => include_str!("./inputs/test_2.txt"),
        3 => include_str!("./inputs/test_3.txt"),
        4 => include_str!("./inputs/test_4.txt"),
        5 => include_str!("./inputs/test_5.txt"),
        _ => panic!("only numbers 0 to 5 are valid"),
    };
}

pub fn get(number: u8) -> &'static str {
    return get_file(number);
}
