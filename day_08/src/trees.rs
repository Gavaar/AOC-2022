use std::{collections::HashMap};

fn find_score(parsed_grid: &Vec<Vec<u8>>, i: usize, j: usize, trlb: char) -> u32 {
    let tree_height = parsed_grid[i][j];
    let max_size = match trlb {
        't'|'b' => parsed_grid.len() as i8,
        'r'|'l' => parsed_grid[0].len() as i8,
        _ => panic!("Didnt match t b r or l"),
    };

    let mut score: u32 = 0;
    let mut index_being_checked: i8 = match trlb {
        't' => i as i8 - 1,
        'b' => i as i8 + 1,
        'r' => j as i8 + 1,
        'l' => j as i8 - 1,
        _ => panic!("Didnt match t b r or l"),
    };

    loop {
        if index_being_checked <= -1 || index_being_checked >= max_size {
            break;
        }

        score += 1;

        if (match trlb {
            't'|'b' => parsed_grid[index_being_checked as usize][j],
            'r'|'l' => parsed_grid[i][index_being_checked as usize],
            _ => panic!("Didnt match t b r or l"),
        }) >= tree_height {
            break;
        }

        index_being_checked = match trlb {
            't'|'l' => index_being_checked - 1,
            'b'|'r' => index_being_checked + 1,
            _ => panic!("Didnt match t b r or l"),
        };
    }

    return score;
}

pub fn are_trees_visible(parsed_grid: Vec<Vec<u8>>) ->  (HashMap<String, bool>, HashMap<String, u32>) {
    let height = parsed_grid.len();
    let width = parsed_grid[0].len();
    let mut visible_tree_hash: HashMap<String, bool> = HashMap::new();
    let mut tree_score_hash: HashMap<String, u32> = HashMap::new();

    let mut highest_left: Vec<i8> = vec![-1; height as usize];
    let mut highest_right: Vec<i8> = vec![-1; height as usize];
    let mut highest_top: Vec<i8> = vec![-1; width as usize];
    let mut highest_bottom: Vec<i8> = vec![-1; width as usize];

    for i in 0..height {
        for j in 0..width {
            let bi = height - 1 - i;
            let bj = width - 1- j;
            let top_left = parsed_grid[i][j];
            let bottom_right = parsed_grid[bi][bj];
            let top_left_tree_id = format!("{}_{}", i.to_string(), j.to_string());
            let bottom_right_tree_id = format!("{}_{}", bi.to_string(), bj.to_string());

            let top_score = find_score(&parsed_grid, i, j, 't');
            let right_score = find_score(&parsed_grid, i, j, 'r');
            let left_score = find_score(&parsed_grid, i, j, 'l');
            let bottom_score = find_score(&parsed_grid, i, j, 'b');
            let total_score = top_score * right_score * left_score * bottom_score;
            tree_score_hash.insert(top_left_tree_id.clone(), total_score);

            if top_left as i8 > highest_left[i] {
                highest_left[i] = top_left as i8;
                visible_tree_hash.insert(top_left_tree_id.clone(), true);
            };
            if top_left as i8 > highest_top[j] {
                highest_top[j] = top_left as i8;
                visible_tree_hash.insert(top_left_tree_id.clone(), true);
            }
            if bottom_right as i8 > highest_bottom[bj] {
                highest_bottom[bj] = bottom_right as i8;
                visible_tree_hash.insert(bottom_right_tree_id.clone(), true);
            }
            if bottom_right as i8 > highest_right[bi] {
                highest_right[bi] = bottom_right as i8;
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

    return (visible_tree_hash, tree_score_hash);
}
