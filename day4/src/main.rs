use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn range(x: i32, y: i32) -> Vec<i32> {
    let mut list: Vec<i32> = Vec::new();
    for i in x..y + 1 {
        list.push(i);
    }
    list
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut sum = 0;
    for line in lines {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(',').collect();
        let range1: Vec<&str> = line[0].split('-').collect();
        let range2: Vec<&str> = line[1].split('-').collect();
        let a = range(
            range1[0].to_string().parse().unwrap(),
            range1[1].to_string().parse().unwrap(),
        );
        let b = range(
            range2[0].to_string().parse().unwrap(),
            range2[1].to_string().parse().unwrap(),
        );
        let x: HashSet<i32> = a.into_iter().collect();
        let y: HashSet<i32> = b.into_iter().collect();
        if x.len() + y.len() > 0 {
            sum += 1
        }
    }
    print!("{}", sum);
}
