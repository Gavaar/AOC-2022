pub struct RockThrower {
    current: usize,
    rocks: Vec<Vec<(usize, usize)>>,
}

impl RockThrower {
    pub fn next(&mut self) -> Vec<(usize, usize)> {
        let next = self.current % self.rocks.len();

        let rock = self.rocks[next].clone();
        self.current += 1;

        rock
    }

    pub fn called(&self) -> usize {
        self.current
    }
}

// pub fn vec_to_rock(rock: &Vec<(usize, usize)>) -> String {
//     let mut printed = Vec::new();
//     let max_rows = *rock.iter().map(|(_, max)| max).max().unwrap();

//     for row in 0..max_rows {
//         let mut row_str = String::from("");
//         for step in rock {
//             let (curr_min, curr_max) = *step;

//             row_str.push(if curr_max > row && curr_min <= row { '#' } else { '.' });
//         }
//         printed.push(row_str);
//     }

//     printed.reverse();
//     return printed.join("\n");
// }

pub fn get_new_rock_thrower(rocks: Vec<Vec<(usize, usize)>>) -> RockThrower {
    let rock_thrower = RockThrower {
        rocks,
        current: 0,
    };
    rock_thrower
}
