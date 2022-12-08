fn get_test_file() -> &'static str {
    return include_str!("./inputs/test.txt");
}
fn get_day_file() -> &'static str {
    return include_str!("./inputs/day.txt");
}

pub fn file(test: bool) -> &'static str {
    let file = if test { get_test_file() } else { get_day_file() };

    return file;
}

pub fn parse_file(file: &str) -> Vec<Vec<u8>> {
    let rows = file.lines();
    let mut parsed: Vec<Vec<u8>> = Vec::new();

    for (i, row) in rows.enumerate() {
        parsed.push(Vec::new());

        for tree in row.chars() {
            parsed[i].push(tree as u8 - 0x30);
        }
    }

    return parsed;
}
