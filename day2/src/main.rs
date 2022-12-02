use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_score(human_play: &str, ai_play: &str) -> i32 {
    0
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut human_score = 0;
    for line in lines {
        let line_content = line.unwrap();
        let plays: Vec<&str> = line_content.split(' ').collect();
        let human_play = plays[0];
        let ai_play = plays[1];
        let round_score = calculate_score(human_play, ai_play);
        human_score += round_score;
    }
    println!("{}", human_score);
}
