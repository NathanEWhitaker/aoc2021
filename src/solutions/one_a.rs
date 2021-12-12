use std::fs::File;
use std::io::{BufReader, BufRead};

fn file_to_vec(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap()).collect()
}

fn main() {
    let lines = &file_to_vec("input/1/a/input.txt");
    let heights: Vec<i32> = lines.iter().map(|s| s.parse().unwrap()).collect();
    let pairs: Vec<(&i32, &i32)> = heights.iter().zip(heights.iter().skip(1)).collect();
    let increases = pairs.iter().map(|(x, y)| (**x - **y) < 0).filter(|x| *x).count();
    println!("Increases = {}", increases);
}
