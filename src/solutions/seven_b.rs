use advent_of_code_2021::file_to_vec;

fn cost(curr_pos: i32, destination: i32) -> i32 {
    let diff = (curr_pos - destination).abs();
    diff * (diff + 1) / 2
}

fn total_cost(positions: &Vec<i32>, destination: i32) -> i32 {
    positions.iter().map(|p| cost(*p, destination)).sum()
}

fn main() {
    let mut positions: Vec<i32> = file_to_vec("input/7/a.txt")
        .remove(0)
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    positions.sort();

    let (mut min_cost, mut min_position) = (i32::MAX, None); 
    for i in positions[0]..positions[positions.len()-1] {
        let total_cost = total_cost(&positions, i);
        if total_cost < min_cost {
            min_position = Some(i);
            min_cost = total_cost;
        }
    }

    println!("{}, {:?}", min_cost, min_position);
} 
