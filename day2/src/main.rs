use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind::InvalidInput},
};

fn convert_ai_input(ai_play: &str) -> Result<i32, Error> {
    match ai_play {
        "A" => Ok(1),
        "B" => Ok(2),
        "C" => Ok(3),
        _ => Err(Error::new(InvalidInput, "Invalid input")),
    }
}

fn convert_human_input(human_play: &str) -> Result<i32, Error> {
    match human_play {
        "X" => Ok(1), //  i lose
        "Y" => Ok(2), //  i draw
        "Z" => Ok(3), //  i win
        _ => Err(Error::new(InvalidInput, "Invalid Input!")),
    }
}

fn get_option_score(human_play: i32) -> i32 {
    match human_play {
        1 => 1,
        2 => 2,
        3 => 3,
        _ => 0,
    }
}
/// 1=> rock 2=>paper 3=> scissors
/// 0 => lose
/// 3 => draw
/// 6 => win
fn calculate_round_score(human_play: i32, ai_play: i32) -> Result<i32, Error> {
    let score_round = 0;
    // score_round += get_option_score(human_play);
    let options = (human_play, ai_play);
    match options {
        (1, 1) => Ok(score_round + get_option_score(3)), // lose
        (1, 2) => Ok(score_round + get_option_score(1)), // lose
        (1, 3) => Ok(score_round + get_option_score(2)), // lose
        (2, 1) => Ok(score_round + 3 + get_option_score(ai_play)), // draw
        (2, 2) => Ok(score_round + 3 + get_option_score(ai_play)), // draw
        (2, 3) => Ok(score_round + 3 + get_option_score(ai_play)), // draw
        (3, 1) => Ok(score_round + 6 + get_option_score(2)),
        (3, 2) => Ok(score_round + 6 + get_option_score(3)),
        (3, 3) => Ok(score_round + 6 + get_option_score(1)),

        _ => Err(Error::new(InvalidInput, "Something happened")),
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut human_score = 0;
    for line in lines {
        let line_content = line.unwrap();
        let plays: Vec<&str> = line_content.split(' ').collect();
        let human_play = convert_human_input(plays[1]).unwrap();
        let ai_play = convert_ai_input(plays[0]).unwrap();
        let round_score = calculate_round_score(human_play, ai_play).unwrap();
        human_score += round_score;
    }
    println!("{}", human_score);
}
