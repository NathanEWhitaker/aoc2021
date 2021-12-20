use advent_of_code_2021::file_to_vec;

fn median(positions: &Vec<i32>) -> i32 {
    if positions.len() % 2 == 0 {
        let med_low = positions[positions.len() / 2];
        let med_high = positions[positions.len() / 2 + 1];
        (med_low + med_high) / 2
    } else {
        positions[positions.len() / 2 + 1]
    }
}

fn main() {
    let mut positions: Vec<i32> = file_to_vec("input/7/a.txt")
        .remove(0)
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    positions.sort();

    let median = median(&positions);
    let cost: i32 = positions.iter().map(|p| (p - median).abs()).sum();
    println!("{}", cost);
}
