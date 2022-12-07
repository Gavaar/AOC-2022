use std::collections::HashMap;

fn get_file(test: bool) -> &'static str {
    if test {
        return include_str!("./inputs/test.txt");
    }

    return include_str!("./inputs/day.txt");
}

// command, param, dir_name || file_name, file_size
fn get_line_value(line: &str) -> (&str, &str, &str, u32) {
    if line.starts_with("$ ") {
        let mut command_input = line.split("$ ").nth(1).unwrap().split(" ");
        let command = command_input.next().unwrap();
        let param = if command == "cd" {
            command_input.next().unwrap()
        } else {
            ""
        };
        return (command, param, "", 0);
    }

    if line.starts_with("dir ") {
        let dir_name = line.split("dir ").nth(1).unwrap();
        return ("", "", dir_name, 0);
    }

    let mut command_input = line.split(' ');
    let file_size = command_input.next().unwrap().parse::<u32>().unwrap();
    let file_name = command_input.next().unwrap();
    return ("", "", file_name, file_size);
}

fn parse_file(test: bool) -> HashMap<String, u32> {
    let output_lines = get_file(test).lines();
    let mut curr_cd = String::from("");
    let mut tree: HashMap<String, u32> = HashMap::new();

    for line in output_lines {
        let (command, param, name, size) = get_line_value(line);

        if command == "cd" {
            if param == "/" {
                curr_cd = String::from("");
                continue;
            }

            if param == ".." {
                let old_cd = curr_cd.clone();
                let mut new_cd = old_cd.split("/").collect::<Vec<&str>>();
                new_cd.pop();

                curr_cd = new_cd.join("/");
                continue;
            }

            curr_cd.push('/');
            curr_cd.push_str(param);

            if !tree.contains_key(&curr_cd) {
                tree.insert(String::from(&curr_cd), 0);
            }

            continue;
        }

        if size == 0 && !name.is_empty() {
            let mut child_dir = curr_cd.clone();
            child_dir.push('/');
            child_dir.push_str(name);

            tree.insert(child_dir, 0);

            continue;
        }

        if size > 0 {
            let mut file_name = curr_cd.clone();
            file_name.push('/');
            file_name.push_str("__");
            file_name.push_str(name);

            let mut relevant_tree = String::from("/");

            for dir_or_file in file_name.split('/') {
                if relevant_tree.chars().last().unwrap() != '/' {
                    relevant_tree.push('/');
                }
                relevant_tree.push_str(dir_or_file);

                let current_dir_size = match tree.get(&relevant_tree) {
                    Some(v) => *v,
                    None => 0,
                };

                tree.insert(String::from(&relevant_tree), current_dir_size + size);
            }
        }
    }

    return tree;
}

pub fn test_file() -> HashMap<String, u32> {
    return parse_file(true);
}
pub fn day_file() -> HashMap<String, u32> {
    return parse_file(false);
}
