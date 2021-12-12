use std::io::{BufRead, BufReader};
use std::fs;
use std::collections::HashMap;

const UP: &str = "up";
const DOWN: &str = "down";
const FORWARD: &str = "forward";
const HORIZONTAL: &str = "horizontal";
const VERTICAL: &str = "vertical";
const AIM: &str = "aim";

pub fn find_distance_travelled(filename: &str) {
    let reader = BufReader::new(fs::File::open(filename).expect("Something went wrong reading the file"));

    let mut dist_accumulator = HashMap::from([
        (HORIZONTAL, 0),
        (VERTICAL, 0),
        (AIM, 0),
    ]);

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let direction= line[0];
        let value= line[1].parse::<i32>().unwrap();
        match direction {
            FORWARD => {
                *dist_accumulator.entry(HORIZONTAL).or_insert(0) += value;
                *dist_accumulator.entry(VERTICAL).or_insert(0) += dist_accumulator.get(AIM).unwrap() * value;
            },
            DOWN => {
                // *dist_accumulator.entry(VERTICAL).or_insert(0) += value;
                *dist_accumulator.entry(AIM).or_insert(0) += value;
            },
            UP => {
                // *dist_accumulator.entry(UP).or_insert(0) -= value;
                *dist_accumulator.entry(AIM).or_insert(0) -= value;
            },
            _ => println!("huh ... shouldn't be here"),
        };
    }
    println!("{}", dist_accumulator.get(HORIZONTAL).unwrap() * dist_accumulator.get(VERTICAL).unwrap());
}
