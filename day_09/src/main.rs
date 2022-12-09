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

fn main() {
    let test_file = files::file(true);
    let day_file = files::file(false);

    println!("#### PART 1 ####");
    println!("Test:");
    part_one(test_file);
    println!("Day:");
    part_one(day_file);
    println!("----------------");
}
