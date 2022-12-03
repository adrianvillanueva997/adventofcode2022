use std::{
    fs::File,
    io::{BufRead, BufReader},
};

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut priority = 0;
    let mut items: Vec<String> = Vec::new();
    for line in lines {
        let line_content = line.unwrap();
        items.push(line_content);
    }
    let items = items.chunks(3);
    for chunk in items {
        let mut matches = false;
        let mut i = 0;
        let first = chunk[0].clone();
        let second = chunk[1].clone();
        let third = chunk[2].clone();
        while !matches && i < first.len() {
            let current_char = first.chars().nth(i).unwrap();
            if second.contains(current_char) && third.contains(current_char) {
                matches = true;
                if current_char.is_lowercase() {
                    let lower_priority =
                        ASCII_LOWER.iter().position(|&i| i == current_char).unwrap() as i32 + 1;
                    priority += lower_priority;
                }
                if current_char.is_uppercase() {
                    let upper_priority =
                        ASCII_UPPER.iter().position(|&i| i == current_char).unwrap() as i32
                            + 1
                            + 26;
                    priority += upper_priority;
                }
            }
            i += 1;
        }
    }
    println!("{}", priority);
}
