fn get_file(test: bool) -> Vec<Vec<&'static str>> {
    let base_file: &'static str = if test {
        include_str!("./inputs/test.txt")
    } else {
        include_str!("./inputs/day.txt")
    };

    return base_file
        .lines()
        .map(|line| {
            line.split(",").collect()
        })
        .collect();
}

pub fn test_file() -> Vec<Vec<&'static str>> {
    return get_file(true);
}

pub fn day_file() -> Vec<Vec<&'static str>> {
    return get_file(false);
}
