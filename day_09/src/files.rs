fn get_test_file() -> &'static str {
    return include_str!("./inputs/test.txt");
}
fn get_day_file() -> &'static str {
    return include_str!("./inputs/day.txt");
}
pub fn bigger() -> &'static str {
    return include_str!("./inputs/bigger_test.txt");
}

pub fn file(test: bool) -> &'static str {
    let file = if test { get_test_file() } else { get_day_file() };

    return file;
}
