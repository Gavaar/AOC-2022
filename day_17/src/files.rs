// Files
fn rock_file() -> &'static str {
    include_str!("./inputs/rocks.txt")
}
fn test_file() -> &'static str {
    include_str!("./inputs/test.txt")
}
fn day_file() -> &'static str {
    include_str!("./inputs/day.txt")
}

fn get_rocks() -> Vec<Vec<(usize, usize)>> {
    let mut rocks: Vec<Vec<(usize, usize)>> = Vec::new();

    for rock in rock_file().split("\n\n") {
        let rock_lines = rock.lines();
        let rock_width_len = rock_lines.clone().collect::<Vec<&str>>()[0]
            .chars()
            .collect::<Vec<char>>()
            .len();
        let rock_lines_len = rock_lines.clone().collect::<Vec<&str>>().len();
        let mut mins: Vec<usize> = vec![9; rock_width_len];
        let mut maxs: Vec<usize> = vec![0; rock_width_len];

        for (rock_height_index, rock_line) in rock_lines.enumerate() {
            let rock_height = rock_lines_len - rock_height_index - 1;

            for (rock_piece_index, rock_piece) in rock_line.chars().enumerate() {
                if rock_piece == '#' {
                    if mins[rock_piece_index] > rock_height {
                        mins[rock_piece_index] = rock_height;
                    }
                    if maxs[rock_piece_index] < rock_height {
                        maxs[rock_piece_index] = rock_height;
                    }
                }
            }
        }

        let mut parsed_rock: Vec<(usize, usize)> = Vec::new();
        for rock_index in 0..rock_width_len {
            parsed_rock.push((mins[rock_index], maxs[rock_index] + 1));
        }

        rocks.push(parsed_rock);
    }

    rocks
}

pub fn get(test: bool) -> (Vec<Vec<(usize, usize)>>, &'static str) {
    let rocks = get_rocks();
    let file = if test { test_file() } else { day_file() };
    (rocks, file)
}
