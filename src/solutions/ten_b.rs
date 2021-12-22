use advent_of_code_2021::file_to_vec;
use std::collections::VecDeque;

enum CorruptionKind {
    Incomplete(Vec<char>),
    Corrupted(char),
    NotCorrupt
}

impl CorruptionKind {
    fn score(&self) -> u64 {
        match self {
            CorruptionKind::Incomplete(chars) => {
                let mut score = 0;
                for c in chars {
                    score *= 5;
                    let points = match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!()
                    };
                    score += points;
                }
                score
            },
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
        let mut completing = Vec::new();
        while let Some(c) = paren_stack.pop_front() {
            let matching = match c {
                '{' => '}',
                '(' => ')',
                '<' => '>',
                '[' => ']',
                _ => panic!()
            };
            completing.push(matching);
        }
        CorruptionKind::Incomplete(completing)
    } else {
        CorruptionKind::NotCorrupt
    }
}


fn main() {
    let mut scores: Vec<u64> = file_to_vec("input/10/a.txt")
        .iter()
        .map(|s| find_corruption(s))
        .filter_map(|kind| {
            match kind {
                CorruptionKind::Incomplete(_) => Some(kind.score()),
                _ => None
            }
        })
        .collect();
    scores.sort();
    println!("{:?}", scores[scores.len() / 2]);

}

