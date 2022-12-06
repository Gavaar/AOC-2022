use std::collections::HashSet;

mod files;

fn marker_position(input: &'static str, expected_marker_length: usize) -> u32 {
    let mut found_index = 0;

    for index in 0..input.len() {
        let for_hash = if index >= expected_marker_length {
            input.get(index - expected_marker_length..index).unwrap()
        } else {
            ""
        };

        let set: HashSet<char> = HashSet::from_iter(for_hash.chars());

        if set.len() >= expected_marker_length {
            found_index = index;
            break;
        }
    }

    return found_index as u32;
}

fn main() {
    assert_eq!(7, marker_position(files::get(1), 4));
    assert_eq!(5, marker_position(files::get(2), 4));
    assert_eq!(6, marker_position(files::get(3), 4));
    assert_eq!(10, marker_position(files::get(4), 4));
    assert_eq!(11, marker_position(files::get(5), 4));

    println!(
        "The marker position for 4 matches is: {}",
        marker_position(files::get(0), 4)
    );

    assert_eq!(19, marker_position(files::get(1), 14));
    assert_eq!(23, marker_position(files::get(2), 14));
    assert_eq!(23, marker_position(files::get(3), 14));
    assert_eq!(29, marker_position(files::get(4), 14));
    assert_eq!(26, marker_position(files::get(5), 14));
    
    println!(
        "The marker position for 14 matches is: {}",
        marker_position(files::get(0), 14)
    );
}
