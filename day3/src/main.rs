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
    for line in lines {
        let line_content = line.unwrap();
        let (first, second) = line_content.split_at(line_content.len() / 2);
        let first: Vec<char> = first.chars().collect();
        let second: Vec<char> = second.chars().collect();
        let mut matches = false;
        let mut i = 0;
        let mut j = 0;
        while !matches && i < first.len() {
            let first_current_value = first[i];
            while !matches && j < second.len() {
                let second_current_value = second[j];
                if first_current_value == second_current_value {
                    if first_current_value.is_lowercase() {
                        let lower_priority = ASCII_LOWER
                            .iter()
                            .position(|&i| i == first_current_value)
                            .unwrap() as i32
                            + 1;
                        priority += lower_priority;
                    }
                    if first_current_value.is_uppercase() {
                        let upper_priority = ASCII_UPPER
                            .iter()
                            .position(|&i| i == first_current_value)
                            .unwrap() as i32
                            + 1
                            + 26;
                        priority += upper_priority;
                    }
                    matches = true;
                }
                j += 1;
            }
            i += 1;
            j = 0;
        }
    }
    println!("{}", priority);
}
