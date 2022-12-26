use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file(filename: &str) -> File {
    let file: File = match File::open(&filename) {
        Ok(file) => file,
        Err(err) => {
            panic!("Error opening {}: {}", filename, err);
        }
    };

    return file;
}

fn get_play_as_int(play: char, base: char) -> i32 {
    return ((play as u32) - (base as u32) + 1) as i32;
}

fn score_round(opp_play: char, strat_play: char, day2: bool) -> i32 {
    let opp_num = get_play_as_int(opp_play, 'A');
    let mut my_num = get_play_as_int(strat_play, 'X');

    if day2 {
        // strat is the desired result on day2
        // X = LOSE (want to be opp - 1)
        // Y = DRAW (want to be opp)
        // Z = WIN (want to be opp + 1)
        my_num = (opp_num + my_num - 2).rem_euclid(3);
        if my_num == 0 {
            my_num = 3;
        }
    }

    let outcome = (opp_num - my_num).rem_euclid(3);
    let outcome_score = match outcome {
        0 => 3, // draw
        1 => 0, // lose
        2 => 6, // win, e.g. I play rock, they play paper
        _ => panic!(),
    };

    return my_num + outcome_score;
}

fn main() {
    assert!(process("test", false) == 15, "Failed testcase");
    assert!(process("test", true) == 12, "Failed testcase");
    process("input", true);
}

fn process(input: &str, day2: bool) -> i32 {
    let reader = BufReader::new(get_file(input));

    // Round 1:
    // An elf gave us a strategy guide with predictions and recommendations
    // for a rock paper scissors tourney. Highest score wins.
    //
    // Score is shape_score + outcome_score
    // shape_score 1 rock, 2 paper, 3 scissors
    // outcome_score 0 lose, 3 draw, 6 win
    let mut score = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let mut round_iter = line.split_whitespace();
        let opp_play = round_iter.next().unwrap().chars().next().unwrap();
        let strat_play = round_iter.next().unwrap().chars().next().unwrap();

        score += score_round(opp_play, strat_play, day2);
    }

    println!("{} {}", input, score);
    return score;
}
