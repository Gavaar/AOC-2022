use super::walk;
use std::collections::HashMap;

pub fn climb<'a>(
    paths: &'a mut Vec<Vec<(usize, usize)>>,
    start: (usize, usize),
    target: (usize, usize),
    matrix: &Vec<Vec<u32>>,
) -> &'a mut Vec<Vec<(usize, usize)>> {
    paths.push(Vec::from([start]));
    let mut visited: HashMap<String, bool> = HashMap::new();
    visited.insert(String::from("0_0"), true);

    loop {
        let mut memory_index: usize = 0;

        for _ in 0..paths.len() {
            let curr_index = memory_index.clone();
            let possible_opts = walk::walk(paths[curr_index].last().unwrap(), matrix);
            let new_opts: Vec<&(usize, usize)> = possible_opts
                .iter()
                .filter(|opt| visited.get(&format!("{}_{}", opt.0, opt.1)) == None)
                .collect();

            if new_opts.len() == 0 {
                paths.remove(curr_index);
                continue;
            }

            for _ in 0..new_opts.len() - 1 {
                paths.insert(curr_index, paths[curr_index].clone());
            }

            memory_index += new_opts.len() - 1;

            for j in 0..new_opts.len() {
                let total_index = curr_index + j;
                let new_opt = new_opts[j];

                paths[total_index].push(*new_opt);
                visited.insert(format!("{}_{}", new_opt.0, new_opt.1), true);
            }

            memory_index += 1;
        }

        if paths.len() == 0 {
            break;
        }

        if visited.get(&format!("{}_{}", target.0, target.1)) != None {
            break;
        }
    }

    paths
}
