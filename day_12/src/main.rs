mod files;
mod walk;
mod climb;

fn part_one(file: &'static str) {
    let (start, target, height_matrix, _) = files::parse_input(file);
    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();
    climb::climb(&mut paths, start, target, &height_matrix);
    
    let smallest = walk::smallest(&paths);
    println!("The smallest is: {:?}", smallest);
    println!("With a size of: {}", smallest.len() - 1);
}

fn part_two(file: &'static str) {
    let (_, target, height_matrix, all_possible_starts) = files::parse_input(file);
    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();

    for start in all_possible_starts {
        let mut paths_for_start: Vec<Vec<(usize, usize)>> = Vec::new();
        climb::climb(&mut paths_for_start, start, target.clone(), &height_matrix);

        if paths_for_start.len() == 0 {
            continue;
        }

        let smallest_path = walk::smallest(&paths_for_start);
        
        paths.push(smallest_path.to_owned());
    }

    let smallest = walk::smallest(&paths);
    println!("The smallest is: {:?}", smallest);
    println!("With a size of: {}", smallest.len() - 1);
}

fn main() {
    let test_file = files::get(true);
    let day_file = files::get(false);

    println!("## PART 1 ##");
    println!("--> Test");
    part_one(test_file);
    println!("--> Day");
    part_one(day_file);
    println!("------------\n");
    println!("## PART 2 ##");
    println!("--> Test");
    part_two(test_file);
    println!("--> Day");
    part_two(day_file);
}
