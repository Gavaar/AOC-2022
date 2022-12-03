use std::{collections::HashMap};

static ROCK: &str = "A";
static PAPER: &str = "B";
static SCIZZORS: &str = "C";

fn input_to_rock_paper_scizors(input: &str) -> u32 {
    return match input {
        // "X"|"A" => 0,
        "Y" | "B" => 1,
        "Z" | "C" => 2,
        _ => 0,
    };
}

fn hand_rules(hand: &str) -> HashMap<&str, &str> {
    return HashMap::from(
        match hand {
            "A" => [
                ("loses", PAPER),
                ("wins", SCIZZORS),
            ],
            "B" => [
                ("loses", SCIZZORS),
                ("wins", ROCK),
            ],
            "C" => [
                ("loses", ROCK),
                ("wins", PAPER),
            ],
            x => panic!("Unexpected invalid token {:?}", x),
        }
    );
}

pub fn needed_to_win<'input, 'his_hand>(input: &'input str, his_hand: &'his_hand str) -> &'his_hand str {
    return match input {
        "X" => hand_rules(his_hand).get(&"wins").copied().unwrap(),
        "Y" => his_hand,
        "Z" => hand_rules(his_hand).get(&"loses").copied().unwrap(),
        x => panic!("Unexpected invalid token {:?}", x),
    };
}

/*
   0 - rock
   1 - paper
   2 - scizzors

   ** CASES **
   him me      =
   0 - 0       0       -
   0 - 1       -1      me
   0 - 2       -2      him
   1 - 0       1       him
   2 - 0       2       me
   1 - 1       0       -
   1 - 2       -1      me
   2 - 1       1       him
   2 - 2       0       -

   ** him - me = result **
   [me] == -1, 2
   [him] == -2, 1
*/
pub fn get_winner(him: &str, me: &str) -> u32 {
    let him_in_num = input_to_rock_paper_scizors(him);
    let me_in_num = input_to_rock_paper_scizors(me);

    let match_result: i32 = him_in_num as i32 - me_in_num as i32;

    return match match_result {
        0 => 3,
        // -2|1 => 0,
        -1 | 2 => 6,
        _ => 0,
    } as u32;
}

pub fn get_hand_score(hand: &str) -> u32 {
    return match hand {
        // "X"|"A" => 1,
        "Y" | "B" => 2,
        "Z" | "C" => 3,
        _ => 1,
    };
}
