use advent_of_code_2021::file_to_vec;
use std::collections::VecDeque;

enum CorruptionKind {
    Incomplete,
    Corrupted(char),
    NotCorrupt
}

impl CorruptionKind {
    fn score(&self) -> u32 {
        match self {
            CorruptionKind::Incomplete => 0,
            &CorruptionKind::NotCorrupt => 0,
            &CorruptionKind::Corrupted(c) => match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!()
            }
        }
    }
}


fn find_corruption(line: &str) -> CorruptionKind {
    let mut paren_stack: VecDeque<char> = VecDeque::new();

    for c in line.chars() {
        let valid = match c {
            '{' | '[' | '<' | '(' => {paren_stack.push_front(c); true},
            '}' => match paren_stack.front() {
                Some('{') => {paren_stack.pop_front(); true},
                _ => {false}
            },
            ']' => match paren_stack.front() {
                Some('[') => {paren_stack.pop_front(); true},
                _ => {false}
            },
            '>' => match paren_stack.front() {
                Some('<') => {paren_stack.pop_front(); true},
                _ => {false}
            },
            ')' => match paren_stack.front() {
                Some('(') => {paren_stack.pop_front(); true},
                _ => {false}
            },
            _ => panic!()
        };

        // gross early return
        if !valid {
            return CorruptionKind::Corrupted(c);
        }
    }

    if !paren_stack.is_empty() {
        CorruptionKind::Incomplete
    } else {
        CorruptionKind::NotCorrupt
    }
}


fn main() {
    println!("{:?}", file_to_vec("input/10/a.txt").iter().map(|s| find_corruption(s).score()).sum::<u32>());

}

