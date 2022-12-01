use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut current_count = 0;
    let mut current_max = 0;
    for line in lines {
        let line_content = line.unwrap();
        if line_content.is_empty() {
            if current_count > current_max {
                current_max = current_count;
            }
            current_count = 0;
        } else {
            let line_number: i32 = line_content.parse().unwrap();
            current_count += line_number;
        }
    }
    println!("{}", current_max);
}
