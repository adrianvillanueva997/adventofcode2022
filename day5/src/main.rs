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
        let index_origin = self.list[crate_origin].len() - 1;
        let element_deleted = self.list[crate_origin].remove(index_origin);
        self.list[crate_destination].push(element_deleted);
        self
    }

    fn move_multiple_crates(
        mut self,
        crate_origin: usize,
        crate_destination: usize,
        crates_to_move: usize,
    ) -> Crates {
        let mut tmp: Vec<String> = Vec::new();
        for _ in 0..crates_to_move {
            let index_origin = self.list[crate_origin].len() - 1;
            let element_deleted = self.list[crate_origin].remove(index_origin);
            tmp.push(element_deleted);
        }
        for i in (0..tmp.len()).rev() {
            let element_to_insert = tmp[i].clone();
            self.list[crate_destination].push(element_to_insert);
        }
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
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    for line in lines {
        let line = line.unwrap();
        if !line.contains("move") {
            continue;
        }
        let line: Vec<&str> = line.split(' ').collect();
        let crates_to_move: usize = line[1].parse().unwrap();
        let crate_origin: usize = line[3].parse().unwrap();
        let crate_destination: usize = line[5].parse().unwrap();
        if crates_to_move == 1 {
            crates = crates.move_crates(crate_origin - 1, crate_destination - 1);
        } else {
            crates = crates.move_multiple_crates(
                crate_origin - 1,
                crate_destination - 1,
                crates_to_move,
            );
        }
    }
    // println!("{:?}", crates.list);
    for i in crates.list {
        print!("{}", i[i.len() - 1]);
    }
}
