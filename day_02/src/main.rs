mod files;
use files::get_file;
use files::get_test_file;

mod rps_match;

mod get_total_score;
use get_total_score::get_total_score;

fn part_one(which: &str) {
    let strategy_test: Vec<&'static str> = if which == "test" {
        get_test_file()
    } else {
        get_file()
    };

    println!(
        "The total score for this strategy would be: {:?}",
        get_total_score(false, strategy_test),
    );
}

fn part_two(which: &str) {
    let strategy_test: Vec<&'static str> = if which == "test" {
        get_test_file()
    } else {
        get_file()
    };

    println!(
        "The total score for this strategy would be: {:?}",
        get_total_score(true, strategy_test),
    );
}

fn main() {
    println!("## Part 1");
    part_one("test");
    part_one("day");
    println!("----------------");

    println!("## Part 2");
    part_two("test");
    part_two("day");
}
