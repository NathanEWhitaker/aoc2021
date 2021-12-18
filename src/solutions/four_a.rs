use advent_of_code_2021::file_to_vec;
use std::convert::TryInto;
use std::collections::HashMap;


#[derive(Debug)]
struct BingoCell {
    val: i32,
    marked: bool
}

#[derive(Debug)]
struct BingoBoard {
    cells: [BingoCell; 25] 
}

impl BingoBoard {
    fn new(rows: Vec<Vec<i32>>) -> Self {
        let cells: [BingoCell; 25] = rows.into_iter()
            .flatten()
            .map(|v| BingoCell{val: v, marked: false})
            .collect::<Vec<BingoCell>>()
            .try_into()
            .unwrap();
        BingoBoard{cells}
    }

    fn row_n_filled(&self, row: usize) -> bool {
        let row_start = row * 5;
        let filled = self.cells[row_start..row_start+5].iter().all(|c| c.marked);
        if filled {
            dbg!(self.cells[row_start..row_start+5].iter().collect::<Vec<_>>());
        }
        filled
    }

    fn col_n_filled(&self, col: usize) -> bool {
        for i in (col..=col+20).step_by(5) {
            if !self.cells[i].marked {
                return false
            }
        }
        for i in (col..col+20).step_by(5) {
            dbg!(&self.cells[i]);
        }
        return true;
    }

    fn winning(&self, winning_ball: i32) -> Option<i32> {
        let is_winning = (0..5).any(|r| self.row_n_filled(r)) || (0..5).any(|c| self.col_n_filled(c));
        if !is_winning {
            None
        } else {
            let sum: i32 = self.cells.iter().filter(|c| !c.marked).map(|c| c.val).sum();
            let score: i32 = winning_ball * sum;
            Some(score)
        }

    }

    fn val_to_cell_map(&self) -> HashMap<i32, Vec<usize>> {
        let mut map = HashMap::new();
        for (i, cell) in self.cells.iter().enumerate() {
             map.entry(cell.val).or_insert_with(|| Vec::new()).push(i);
        }
        map
    }

    fn mark_cell(&mut self, pos: usize) {
        self.cells[pos].marked = true
    }

    fn print_board(&self) {
        let vals: Vec<String> = self.cells.iter().map(|c| if c.marked { String::from("*") } else { c.val.to_string() }).collect();
        vals.chunks(5).for_each(|row| {
            println!("{}", row.join(","));
        });
    }

}

fn board_lines_to_board(mut lines: &[String]) -> BingoBoard {
    BingoBoard::new(lines.iter().map(|l| {
        l.split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }).collect())
}


fn construct_val_to_cell(boards: &Vec<BingoBoard>) -> HashMap<i32, Vec<(usize, usize)>> {
    let mut final_map: HashMap<i32, Vec<(usize, usize)>> = HashMap::new(); 
    boards.iter()
        .map(|b| b.val_to_cell_map())
        .enumerate()
        .fold(&mut final_map, |acc, (b_idx, mut map)| {
            for (k, c_idxs) in map.iter_mut() {
                acc.entry(*k).or_insert_with(|| Vec::new()).append(&mut c_idxs.iter().map(|c_idx| (b_idx, *c_idx)).collect());
            }
            acc
    });
    final_map
}

fn main() {
    let mut lines: Vec<String> = file_to_vec("input/4/a.txt").into_iter().filter(|s| !s.is_empty()).collect();
    let balls: Vec<i32> = lines.remove(0).split(",").map(|bs| bs.parse().unwrap()).collect();
    let mut boards: Vec<BingoBoard> = lines.chunks(5).map(|b| board_lines_to_board(b)).collect();
    let val_to_cells = construct_val_to_cell(&boards);
    // run first five iteratitns since no solves are possible
    for i in 0..5 {
        let ball = balls[i];
        val_to_cells.get(&ball).unwrap().iter().for_each(|(b, c)| {
            boards[*b].mark_cell(*c);
        })
    }

    let mut first_winning_score: Option<i32> = None;
    for i in 5..(balls.len()) {
        let ball = balls[i];
        val_to_cells.get(&ball).unwrap().iter().for_each(|(b, c)| {
            let board = &mut boards[*b];
            board.mark_cell(*c);
            if first_winning_score.is_none() {
                if let Some(score) = board.winning(ball) {
                    first_winning_score = Some(score);
                    dbg!(ball);
                    board.print_board();
                }
            }
        });
        // Ideally I would break when first winning board is found, but I can't break inside
        // closuer without the compiler complaining
        if first_winning_score.is_some() {
            break;
        }
    }

    println!("First winning score {:?}", first_winning_score);

}
