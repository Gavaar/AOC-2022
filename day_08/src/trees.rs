use std::{collections::HashMap};

pub fn are_trees_visible(parsed_grid: Vec<Vec<u8>>) ->  HashMap<String, bool> {
    let height = parsed_grid.len();
    let width = parsed_grid[0].len();
    let mut visible_tree_hash: HashMap<String, bool> = HashMap::new();

    let mut highest_left: Vec<i8> = vec![-1; height as usize];
    let mut highest_right: Vec<i8> = vec![-1; height as usize];
    let mut highest_top: Vec<i8> = vec![-1; width as usize];
    let mut highest_bottom: Vec<i8> = vec![-1; width as usize];

    for i in 0..height {
        for j in 0..width {
            let top_left = parsed_grid[i][j];
            let bottom_right = parsed_grid[height - 1 - i][width - 1 - j];
            let top_left_tree_id = format!("{}_{}", i.to_string(), j.to_string());
            let bottom_right_tree_id = format!("{}_{}", (height - 1 - i).to_string(), (width - 1 - j).to_string());

            if top_left as i8 > highest_left[i] {
                highest_left[i] = top_left as i8;
                visible_tree_hash.insert(top_left_tree_id.clone(), true);
            };
            if top_left as i8 > highest_top[j] {
                highest_top[j] = top_left as i8;
                visible_tree_hash.insert(top_left_tree_id.clone(), true);
            }
            if bottom_right as i8 > highest_bottom[width - 1 - j] {
                highest_bottom[width - 1 - j] = bottom_right as i8;
                visible_tree_hash.insert(bottom_right_tree_id.clone(), true);
            }
            if bottom_right as i8 > highest_right[height - 1 - i] {
                highest_right[height - 1 - i] = bottom_right as i8;
                visible_tree_hash.insert(bottom_right_tree_id.clone(), true);
            }

            if !visible_tree_hash.contains_key(&top_left_tree_id) {
                visible_tree_hash.insert(top_left_tree_id.clone(), false);
            }
            if !visible_tree_hash.contains_key(&bottom_right_tree_id) {
                visible_tree_hash.insert(bottom_right_tree_id.clone(), false);
            }
        }
    }

    return visible_tree_hash;
}
