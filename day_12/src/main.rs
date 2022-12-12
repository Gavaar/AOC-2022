mod files;
mod walk;
mod climb;

fn part_one(file: &'static str) {
    let (start, target, height_matrix) = files::parse_input(file);
    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();
    climb::climb(&mut paths, start, target, height_matrix);
    
    // println!("All walkable paths are:");
    // for path in &paths {
    //     println!("{:?}\n", path);
    // }
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
}
