fn get_adjacent(pos: &(usize, usize), matrix: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut adjacent: Vec<(usize, usize)> = Vec::new();
    let columns = matrix[pos.1].len() as i32;
    let rows = matrix.len() as i32;

    let left = (pos.0 as i32 - 1, pos.1 as i32);
    let right = (pos.0 as i32 + 1, pos.1 as i32);
    let bottom = (pos.0 as i32, pos.1 as i32 + 1);
    let top = (pos.0 as i32, pos.1 as i32 - 1);

    for new_pos in [left,right,bottom,top] {
        if new_pos.0 <= -1 || new_pos.0 >= columns || new_pos.1 <= -1 || new_pos.1 >= rows {
            continue;
        }

        adjacent.push((new_pos.0 as usize, new_pos.1 as usize));
    }

    return adjacent;
}

pub fn walk(pos: &(usize, usize), matrix: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut can_move_to: Vec<(usize, usize)> = Vec::new();
    let curr_height = matrix[pos.1][pos.0];
    let adjacent = get_adjacent(&pos, &matrix);

    for tile in adjacent {
        if (matrix[tile.1][tile.0] as i32 - curr_height as i32) >= 2 {
            continue;
        }

        can_move_to.push(tile);
    }

    can_move_to
}

pub fn smallest(paths: &Vec<Vec<(usize, usize)>>) -> &Vec<(usize, usize)> {
    let mut smallest_index: usize = 0;
    let mut smallest_length: usize = 99999;

    for (index, path) in paths.iter().enumerate() {
        if path.len() < smallest_length {
            smallest_length = path.len();
            smallest_index = index;
        }
    }

    return &paths[smallest_index];
}