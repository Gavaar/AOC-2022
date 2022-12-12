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

// start, target, matrix as [Y[X]]
pub fn parse_input(file: &'static str) -> ((usize, usize), (usize, usize), Vec<Vec<u32>>, Vec<(usize, usize)>) {
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let mut all_possible_starts: Vec<(usize, usize)> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut target: (usize, usize) = (0,0);

    for (y, line) in file.lines().enumerate() {
        let mut row: Vec<u32> = Vec::new();

        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (x, y);
                all_possible_starts.push(start);
                row.push(1);
                continue;
            }
            if char == 'E' {
                target = (x, y);
                row.push(26);
                continue;
            }

            if char == 'a' {
                all_possible_starts.push((x, y));
            }

            row.push((char as u32) - 96);
        }

        matrix.push(row);
    }

    return (start, target, matrix, all_possible_starts);
}