use std::fs::File;
use std::io::{BufReader,BufRead};

pub fn load_input(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("file not found");
    let reader = BufReader::new(file);
    return reader.lines().map(|l| l.unwrap()).collect();
}