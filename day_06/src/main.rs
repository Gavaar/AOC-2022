use std::collections::HashSet;

mod files;

fn marker_position(input: &'static str) -> u32 {
    let mut found_index = 0;

    for index in 0..input.len() {
        let for_hash = if index >=4 {input.get(index-4..index).unwrap() } else { "" };
        let set: HashSet<char> = HashSet::from_iter(for_hash.chars());

        if set.len() >= 4 {
            found_index = index;
            println!("found set {:?} at index {}", set, found_index);
            break;
        }
    };

    return found_index as u32;
}

fn main() {
    assert_eq!(7, marker_position(files::get(1)));
    assert_eq!(5, marker_position(files::get(2)));
    assert_eq!(6, marker_position(files::get(3)));
    assert_eq!(10, marker_position(files::get(4)));
    assert_eq!(11, marker_position(files::get(5)));

    println!("The marker position is: {}", marker_position(files::get(0)));
}
