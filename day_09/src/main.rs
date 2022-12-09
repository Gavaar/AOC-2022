use std::collections::HashSet;

mod files;
mod utils;
mod movements;

fn part_one(file: &str) {
    let mut tail: (i32, i32) = (0,0);
    let mut head: (i32, i32) = (0,0);
    let head_movements = utils::parse_files(file);

    let mut head_history: HashSet<String> = HashSet::from([utils::pos_into_str(head)]);
    let mut tail_history: HashSet<String> = HashSet::from([utils::pos_into_str(tail)]);

    for movement in head_movements {
        let direction = movement.0;
        let magnitude = movement.1;

        for _ in 0..magnitude {
            head = movements::move_head(&head, direction);
            tail = movements::move_tail(&tail, &head);

            head_history.insert(utils::pos_into_str(head));
            tail_history.insert(utils::pos_into_str(tail));
        }
    }

    println!("Head has visited {} positions", head_history.len());
    println!("Tail has visited {} positions", tail_history.len());
}

fn part_two(file: &str) {
    let tail_size = 10;
    let head_movements = utils::parse_files(file);
    let mut tails: Vec<(i32, i32)> = vec![(0,0); tail_size]; // 0 is head
    let mut tails_history: Vec<HashSet<String>> = vec![HashSet::from([utils::pos_into_str(tails[0])]); tail_size];

    for movement in head_movements {
        let direction = movement.0;
        let magnitude = movement.1;

        for _ in 0..magnitude {
            for index in 0..tail_size {
                tails[index] = if index == 0 {
                    movements::move_head(&tails[index], direction)
                } else {
                    movements::move_tail(&tails[index], &tails[index - 1])
                };

                tails_history[index].insert(utils::pos_into_str(tails[index]));
            }
        }
    }

    println!("The moved places is: {:?}", tails_history.iter().map(|set| set.len()).collect::<Vec<usize>>());
}

fn main() {
    let test_file = files::file(true);
    let bigger_test_file = files::bigger();
    let day_file = files::file(false);

    println!("#### PART 1 ####");
    println!("Test:");
    part_one(test_file);
    println!("Bigger (for fun):");
    part_one(bigger_test_file);
    println!("Day:");
    part_one(day_file);
    println!("----------------");

    println!("#### PART 2 ####");
    println!("Test:");
    part_two(test_file);
    println!("Bigger:");
    part_two(bigger_test_file);
    println!("Day:");
    part_two(day_file);
    println!("----------------");
}
