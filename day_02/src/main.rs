mod files;
use files::get_file;
use files::get_test_file;

mod rps_match;
use rps_match::get_hand_score;
use rps_match::get_winner;

fn part_one(which: &str) {
    let strategy_test: Vec<&'static str> = if which == "test" {
        get_test_file()
    } else {
        get_file()
    };
    let mut total_score: u32 = 0;

    for contest in strategy_test {
        let plays: Vec<&str> = contest.splitn(2, " ").collect();
        let winner_score = get_winner(plays[0], plays[1]);
        let hand_score = get_hand_score(plays[1]);

        total_score += winner_score + hand_score;
    }

    println!(
        "The total score for this strategy would be: {:?}",
        total_score
    );
}

fn main() {
    println!("## Part 1");
    part_one("test");
    part_one("day");
    println!("----------------");
    println!("## Part 2");
}
