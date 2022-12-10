mod files;
mod cycles;

fn get_relevant_values(signals: &Vec<i32>, from: usize, to: usize, step: usize) -> Vec<i32> {
    let err_value = -999999999;
    let mut result = Vec::new();

    for i in (from..to + 1).step_by(step) {
        let value = match signals.get(i) {
            Some(v) => *v,
            None => err_value,
        };

        if value != err_value {
            result.push(value);
        }
    }

    return result;
}

fn part_one(file: &'static str) {
    let lifecycle = cycles::calc(file);
    let signal_strengths = cycles::into_signal_strength(&lifecycle);

    let relevant = get_relevant_values(&signal_strengths, 20, 220, 40);
    let sum = relevant.iter().fold(0, |acc, r| acc + r);
    println!("The sum of signal strengths is {}", sum);
}

fn main() {
    let test_file = files::get(true);
    let day_file = files::get(false);

    println!("### PART 1 ###");
    println!("-- Test");
    part_one(test_file);
    println!("-- Day");
    part_one(day_file);

    println!("--------------");

    println!("### PART 2 ###");
}
