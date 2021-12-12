use advent_of_code_2021::file_to_vec;


fn zero_bit_in_pos(line: &str, pos: usize) -> bool {
    line.chars().nth(pos).unwrap() == '0'
}

fn do_step(readings: Vec<String>, pos: usize, keep_lesser: bool) -> Vec<String> {
    let partitions: (Vec<String>, Vec<String>) = readings.into_iter().partition(|line| zero_bit_in_pos(line, pos));
    if partitions.0.len() <= partitions.1.len() {
        if keep_lesser {
            partitions.0
        } else {
            partitions.1
        }
    } else {
        if keep_lesser {
            partitions.1
        } else {
            partitions.0
        }
    }
}

fn find_reading(mut readings: Vec<String>, keep_lesser: bool) -> String {
    let mut pos = 0;
    while readings.len() > 1 {
        readings = do_step(readings, pos, keep_lesser);
        pos += 1;
    }
    // not sure if this is idiomatic or not
    readings.remove(0)
}


fn main() {
    let lines: Vec<String> = file_to_vec("input/3/a.txt");
    let co2_reading = usize::from_str_radix(&find_reading(lines.clone(), false), 2).unwrap();
    let o2_reading = usize::from_str_radix(&find_reading(lines, true), 2).unwrap();
    dbg!(co2_reading * o2_reading);
    
}
