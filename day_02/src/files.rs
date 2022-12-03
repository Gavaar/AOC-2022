pub fn get_test_file() -> Vec<&'static str> {
    return include_str!("./inputs/test.txt").split("\n").collect();
}
pub fn get_file() -> Vec<&'static str> {
    return include_str!("./inputs/day.txt").split("\n").collect();
}
