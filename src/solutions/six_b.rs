use advent_of_code_2021::file_to_vec;



fn main() {
    let mut counts = [0usize; 7];
    let mut hatchlings = [0usize; 2];
    let mut zero_ptr = 0usize;
    let mut seven_ptr = 0usize;

    let fishies: Vec<usize> = file_to_vec("input/6/a.txt").remove(0).split(",").map(|s| s.parse::<usize>().unwrap()).collect();
    for fish in fishies {
        counts[fish] += 1;
    }

    for _ in 0..256 {
        let new_hatchlings = counts[zero_ptr];
        let new_fish = hatchlings[seven_ptr];
        let six_ptr = zero_ptr;
        let eight_ptr = seven_ptr;
        zero_ptr = (zero_ptr + 1) % 7;
        seven_ptr = (seven_ptr + 1) % 2;
        counts[six_ptr] += new_fish;
        hatchlings[eight_ptr] = new_hatchlings;
    }

    println!("{:?}", counts.iter().sum::<usize>() + hatchlings.iter().sum::<usize>());
}
