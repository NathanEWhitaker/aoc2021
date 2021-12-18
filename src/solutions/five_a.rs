use advent_of_code_2021::file_to_vec;
use std::collections::HashSet;
use std::cmp::{min, max};

type Board=[[usize; 10]; 10];

#[derive(Debug)]
struct Segment {
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize
} 

impl Segment {
    
    fn from_line(line: &str) -> Self {
        let mut parts = line.split(" -> ").map(Segment::part_to_coord); 
        let start = parts.next().unwrap();
        let end = parts.next().unwrap();
        Segment {
            x_start: start.0,
            y_start: start.1,
            x_end: end.0,
            y_end: end.1
        }
    }

    fn part_to_coord(part: &str) -> (usize, usize) {
        let mut coords = part.split(",").map(|s| s.parse().unwrap());
        (coords.next().unwrap(), coords.next().unwrap())
    }

    fn is_horizontal(&self) -> bool {
        return self.y_start == self.y_end;
    }
    
    fn is_vertical(&self) -> bool {
        return self.x_start == self.x_end
    }
}

fn add_horizontal(x_start: usize, x_end: usize, y_coord: usize, board: &mut [[usize; 10]; 10]) -> Vec<(usize, usize)> {
    let mut overlapping = Vec::new();
    for i in pair_to_range(x_start, x_end) {
        if board[y_coord][i] > 0 {
            overlapping.push((i, y_coord));
        }
        board[y_coord][i] += 1;
    }
    overlapping
}

fn pair_to_range(a: usize, b: usize) -> Box<dyn Iterator<Item=usize> + 'static> {
    let low = min(a, b);
    let high = max(a, b);
    if a < b {
        Box::new(low..=high)
    } else {
        Box::new((low..=high).rev())
    }
}

fn add_vertical(x_coord: usize, y_start: usize, y_end: usize, board: &mut [[usize; 10]; 10]) -> Vec<(usize, usize)> {
    let mut overlapping = Vec::new();
    for i in pair_to_range(y_start, y_end) {
        if board[i][x_coord] > 0 {
            overlapping.push((x_coord, i));
        }
        board[i][x_coord] += 1;
    }
    overlapping
}

fn add_diagonal(x_start: usize, x_end: usize, y_start: usize, y_end: usize, board: &mut Board) -> Vec<(usize, usize)> {
    let mut overlapping = Vec::new();
    for (row, col) in pair_to_range(y_start, y_end).zip(pair_to_range(x_start, x_end)) {
        if board[row][col] > 0 {
            overlapping.push((col, row));
        }
        board[row][col] += 1;
    }
    overlapping
}

fn c_str(c: &usize) -> String {
    if *c == 0 {
       String::from(".")
    } else {
       c.to_string()
    }
}

fn board_to_str(board: &Board) -> String {
    let rowstr: Vec<String> = board.iter().map(|r| r.iter().map(c_str).collect()).collect();
    rowstr.join("\n")
}

fn main() {
    let mut board: Board = [[0usize; 10]; 10];
    let mut overlapping: HashSet<(usize, usize)> = HashSet::new();
    file_to_vec("input/5/test.txt")
        .iter()
        .map(|s| Segment::from_line(s))
        .for_each(|l| {
            if l.is_horizontal() {
                add_horizontal(l.x_start, l.x_end, l.y_start, &mut board).iter().for_each(|o| {overlapping.insert(*o);});
            } else if l.is_vertical() {
                add_vertical(l.x_start, l.y_start, l.y_end, &mut board).iter().for_each(|o| {overlapping.insert(*o);});
            } else {
                add_diagonal(l.x_start, l.x_end, l.y_start, l.y_end, &mut board).iter().for_each(|o| {overlapping.insert(*o);})
            }
        });
    println!("Overlapping {}", overlapping.len());
}
