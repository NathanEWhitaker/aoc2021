use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn file_to_vec(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap()).collect()
}

