use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut current_count = 0;
    let mut v: Vec<i32> = Vec::new();
    for line in lines {
        let line_content = line.unwrap();
        if line_content.is_empty() {
            v.push(current_count);
            current_count = 0;
        } else {
            let line_number: i32 = line_content.parse().unwrap();
            current_count += line_number;
        }
    }
    v.sort();
    v.reverse();
    let total = v[0] + v[1] + v[2];
    println!("{}", total);
}
