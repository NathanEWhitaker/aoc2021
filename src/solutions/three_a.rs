use advent_of_code_2021::file_to_vec;



fn main() {
    let lines = file_to_vec("input/3/a.txt");
    let mut one_counts: Vec<usize> = vec![0; lines[0].len()];
    for line in lines.iter() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '1' => one_counts[i] += 1,
                _ => ()
            }
        }
    }
    let mut gamma = 0;
    let mut beta = 0;
    for (pos, one_count) in one_counts.iter().rev().enumerate() {
        let bit  = 1 << pos;
        if 2 * one_count > lines.len() {
            gamma += bit;
        } else {
            beta += bit;
        }
    }
    //gamma = gamma >> 1;
    //beta = beta >> 1;
    dbg!(one_counts);
    dbg!(format!("{:b}", gamma));
    dbg!(format!("{:012b}", beta));
    dbg!(lines.len() / 2);
    dbg!(gamma * beta);
}
