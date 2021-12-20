use advent_of_code_2021::file_to_vec;

const SEGS_PER_0: i32 = 6;
const SEGS_PER_1: i32 = 2;
const SEGS_PER_2: i32 = 5;
const SEGS_PER_3: i32 = 5;
const SEGS_PER_4: i32 = 4;
const SEGS_PER_5: i32 = 5;
const SEGS_PER_6: i32 = 6;
const SEGS_PER_7: i32 = 3;
const SEGS_PER_8: i32 = 7;
const SEGS_PER_9: i32 = 6;

fn is_easy(signal: &str) -> bool {
    match signal.len() as i32 {
        SEGS_PER_1 | SEGS_PER_4 | SEGS_PER_7 | SEGS_PER_8 => true,
        _ => false
    }
}

fn main() {
    let counts: usize = file_to_vec("input/8/a.txt")
        .iter()
        .flat_map(|s| s.split(" | ").skip(1).next().unwrap().split(" "))
        .filter(|s| is_easy(s)).count();
    println!("Counts {}", counts);
}
