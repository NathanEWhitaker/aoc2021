use advent_of_code_2021::file_to_vec;

struct SubmarinePosition {
    horizontal_pos: i32,
    depth: i32
}

impl SubmarinePosition {
    fn new() -> Self {
        SubmarinePosition{horizontal_pos: 0, depth: 0}
    }
    
    fn do_command(&mut self, c: &Command) {
        match c {
            &Command::Forward(units) => self.horizontal_pos += units,
            &Command::Up(units) => self.depth -= units,
            &Command::Down(units) => self.depth += units
        }
    }
}

#[derive(Debug)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32)
}

fn line_to_command(line: &str) -> Command {
    let parts: Vec<&str> = line.split(' ').take(2).collect();
    match parts[0].to_lowercase().as_str() {
        "forward" => Command::Forward(parts[1].parse().unwrap()),
        "up" => Command::Up(parts[1].parse().unwrap()),
        "down" => Command::Down(parts[1].parse().unwrap()),
        _ => panic!()
    }
}


fn main() {
    let commands: Vec<Command> = file_to_vec("input/2/a.txt").iter().map(|line| line_to_command(line)).collect();
    let mut pos = SubmarinePosition::new();
    for command in commands {
        dbg!(&command);
        pos.do_command(&command);
    }
    println!("Horizontal * depth {}", pos.horizontal_pos * pos.depth);
}

