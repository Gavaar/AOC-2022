pub fn sprite_image(positions: &Vec<i32>) -> String {
    let mut painting = String::from("");

    for (index, pos) in positions.iter().enumerate() {
        if index == 0 {
            continue;
        }

        let index_pos = index % 40;

        if index_pos >= *pos as usize && index_pos <= *pos as usize + 2 {
            painting.push('#');
        } else {
            painting.push('.');
        }

        if index_pos == 0 {
            painting.push('\n');
        }
    }

    painting
}
