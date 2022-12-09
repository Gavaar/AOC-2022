pub fn parse_files(file_str: &str) -> Vec<(&str, i32)> {
    return file_str.lines().map(|line| {
        let mut inputs = line.split(' ');
        return (inputs.next().unwrap(), inputs.next().unwrap().parse::<i32>().unwrap());
    }).collect::<Vec<(&str, i32)>>();
}

pub fn pos_into_str(pos: (i32, i32)) -> String {
    return format!("{}_{}", pos.0, pos.1);
}
