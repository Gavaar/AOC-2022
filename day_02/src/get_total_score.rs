use super::rps_match::get_hand_score;
use super::rps_match::get_winner;
use super::rps_match::needed_to_win;

pub fn get_total_score(part_2: bool, strategy_test: Vec<&'static str>) -> u32 {
    let mut total_score: u32 = 0;

    for contest in strategy_test {
        let plays: Vec<&str> = contest.splitn(2, " ").collect();
        let my_play = if part_2 { needed_to_win(plays[1], plays[0]) } else { plays[1] };
        let winner_score = get_winner(plays[0], my_play);
        let hand_score = get_hand_score(my_play);

        total_score += winner_score + hand_score;
    }

    return total_score;
}
