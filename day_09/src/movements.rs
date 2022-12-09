pub fn move_head(head: &(i32, i32), direction: &str) -> (i32, i32) {
    return match direction {
        "U" => (head.0, head.1 + 1),
        "D" => (head.0, head.1 - 1),
        "L" => (head.0 - 1, head.1),
        "R" => (head.0 + 1, head.1),
        _ => panic!("Direction is not 'U' 'R' 'L' or 'D'"),
    };
}

pub fn move_tail(tail: &(i32, i32), head: &(i32, i32)) -> (i32, i32) {
    let separation_x = head.0 - tail.0;
    let separation_y = head.1 - tail.1;
    let mut new_tail = (tail.0, tail.1);

    if separation_x.abs() >= 2 || separation_y.abs() >= 2 {
        if separation_x.abs() >= 1 {
            new_tail.0 = if separation_x > 0 { new_tail.0 + 1} else { new_tail.0 - 1 };
        }
        if separation_y.abs() >= 1 {
            new_tail.1 = if separation_y > 0 { new_tail.1 + 1} else { new_tail.1 - 1 };
        }
    }

    return new_tail;
}
