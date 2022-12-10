fn test_file() -> &'static str {
    include_str!("./inputs/test.txt")
}

fn day() -> &'static str {
    include_str!("./inputs/day.txt")
}

pub fn get(test: bool) -> &'static str {
    return if test {
        test_file()
    } else {
        day()
    };
}
