use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind::InvalidInput},
};
#[derive(Copy, Clone, PartialEq, Eq)]
enum Options {
    Rock,
    Paper,
    Scissors,
}

fn convert_ai_input(ai_play: &str) -> Result<Options, Error> {
    match ai_play {
        "A" => Ok(Options::Rock),
        "B" => Ok(Options::Paper),
        "C" => Ok(Options::Scissors),
        _ => Err(Error::new(InvalidInput, "Invalid input")),
    }
}

fn convert_human_input(human_play: &str) -> Result<Options, Error> {
    match human_play {
        "X" => Ok(Options::Rock),     //  i lose
        "Y" => Ok(Options::Paper),    //  i draw
        "Z" => Ok(Options::Scissors), //  i win
        _ => Err(Error::new(InvalidInput, "Invalid Input!")),
    }
}

fn get_option_score(human_play: Options) -> i32 {
    match human_play {
        Options::Rock => 1,
        Options::Paper => 2,
        Options::Scissors => 3,
    }
}

/// 0 => lose
/// 3 => draw
/// 6 => win
fn calculate_round_score(human_play: Options, ai_play: Options) -> Result<i32, Error> {
    let score_round = 0;
    // score_round += get_option_score(human_play);
    let options = (human_play, ai_play);
    match options {
        (Options::Rock, Options::Rock) => Ok(score_round + get_option_score(Options::Scissors)), // lose
        (Options::Rock, Options::Paper) => Ok(score_round + get_option_score(Options::Rock)), // lose
        (Options::Rock, Options::Scissors) => Ok(score_round + get_option_score(Options::Paper)), // lose
        (Options::Paper, Options::Rock) => Ok(score_round + 3 + get_option_score(ai_play)), // draw
        (Options::Paper, Options::Paper) => Ok(score_round + 3 + get_option_score(ai_play)), // draw
        (Options::Paper, Options::Scissors) => Ok(score_round + 3 + get_option_score(ai_play)), // draw
        (Options::Scissors, Options::Rock) => {
            Ok(score_round + 6 + get_option_score(Options::Paper))
        }
        (Options::Scissors, Options::Paper) => {
            Ok(score_round + 6 + get_option_score(Options::Scissors))
        }
        (Options::Scissors, Options::Scissors) => {
            Ok(score_round + 6 + get_option_score(Options::Rock))
        }
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
