use std::{
    fs::File,
    io::{BufRead, BufReader},
};
#[derive(Clone)]
struct Crates {
    list: Vec<Vec<String>>,
}

impl Crates {
    fn new(input: Vec<Vec<&str>>) -> Crates {
        let mut x = Crates { list: vec![] };
        for i in input {
            let mut tmp: Vec<String> = Vec::new();
            for j in i {
                tmp.push(String::from(j));
            }
            x.list.push(tmp);
        }
        x
    }
    fn move_crates(
        mut self,
        // crates_to_move: i32,
        crate_origin: usize,
        crate_destination: usize,
    ) -> Crates {
        let element_deleted = self.list[crate_origin].swap_remove(0);
        self.list[crate_destination].push(element_deleted);
        self
    }
}

fn main() {
    let mut crates = Crates::new(vec![
        vec!["S", "Z", "P", "D", "L", "B", "F", "C"],
        vec!["N", "V", "G", "P", "H", "W", "B"],
        vec!["F", "W", "B", "J", "G"],
        vec!["G", "J", "N", "F", "L", "W", "C", "S"],
        vec!["W", "J", "L", "T", "P", "M", "S", "H"],
        vec!["B", "C", "W", "G", "F", "S"],
        vec!["H", "T", "P", "M", "Q", "B", "W"],
        vec!["F", "S", "W", "T"],
        vec!["N", "C", "R"],
    ]);
    // println!("{:?}", crates.list);
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    for line in lines {
        let line = line.unwrap();
        if !line.contains("move") {
            continue;
        }
        println!("{}", line);
        let line: Vec<&str> = line.split(' ').collect();
        let crates_to_move: i32 = line[1].parse().unwrap();
        let crate_origin: usize = line[3].parse().unwrap();
        let crate_destination: usize = line[5].parse().unwrap();
        if crates_to_move == 1 {
            crates = crates.move_crates(crate_origin - 1, crate_destination - 1);
            // println!("{:?}", crates.list);
        } else {
            for _ in 0..crates_to_move {
                crates = crates.move_crates(crate_origin - 1, crate_destination - 1);
                // println!("{:?}", crates.list);
            }
        }
    }
    // println!("{:?}", crates.list);
    for i in crates.list {
        print!("{}", i[i.len() - 1]);
    }
}
