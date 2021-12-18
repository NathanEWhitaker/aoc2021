use advent_of_code_2021::file_to_vec;

fn wrapping_decr(fish: usize) -> usize {
    if fish > 0 {
        fish - 1
    } else {
        6
    }
}

fn tick(fishies: &Vec<usize>) -> Vec<usize> {
    let mut new_fishies: Vec<usize> = Vec::new();
    for fish in fishies.iter() {
        if *fish == 0 {
            new_fishies.push(8);
        }
        new_fishies.push(wrapping_decr(*fish));
    }
    new_fishies
}

fn main() {
    let mut fishies = file_to_vec("input/6/a.txt").remove(0).split(",").map(|s| s.parse::<usize>().unwrap()).collect();
    for _ in 0..80 {
        fishies = tick(&fishies);
    }
    println!("{}", fishies.len());
}
