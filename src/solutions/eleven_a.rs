use advent_of_code_2021::file_to_vec;
use std::marker::PhantomData;

type EnergyLevel = usize;

struct OctoBoard<'a> {
    octos: Vec<EnergyLevel>,
    total_flashes: usize,
    phantom: PhantomData<&'a EnergyLevel>
} 

impl<'a> OctoBoard<'a> {

    fn new(lines: Vec<String>) -> Self {
        let octos = lines.iter().flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize)).collect();
        OctoBoard {octos, total_flashes: 0, phantom: PhantomData}
    }

    fn to_idx(row: i32, col: i32) -> Option<usize> {
        if row < 0 || row > 9 || col < 0 || col > 9 {
            None
        } else {
            Some(row as usize * 10usize + col as usize)
        }
    }

    // I tried to make this an Impl index but you can't make that return an option (unless you
    // already own an option I guess)
    fn index(&self, index: (i32, i32)) -> Option<EnergyLevel> {
        OctoBoard::to_idx(index.0, index.1).map(|idx| self.octos[idx])
    }

    fn neighbors(row: i32, col: i32) -> impl Iterator<Item = Option<usize>> + 'a {
        [(-1, -1), (-1, 0), (-1, 1),
         (0, -1),           (0, 1),
         (1, -1),  (1, 0),   (1, 1)].iter().map(move |delta| { OctoBoard::to_idx(row + delta.0, col + delta.1)})
    }

    fn increase_enegry(&mut self) {
        for i in 0..self.octos.len() {
            self.octos[i] += 1;
        }
    }

    fn inv_idx(idx: usize) -> (i32, i32) {
        ((idx / 10) as i32, (idx % 10) as i32)
    }

    fn step_cell(&mut self, row: i32, col: i32, already_flashed: &mut Vec<bool>) -> Vec<usize> {
        let idx = OctoBoard::to_idx(row, col).unwrap();
        let mut neighbors_which_will_flash = Vec::new();
        if self.octos[idx] > 9 && !already_flashed[idx] {
            self.total_flashes += 1;
            for n_idx in OctoBoard::neighbors(row, col).filter_map(|x| x) {
                if !already_flashed[n_idx] {
                    self.octos[n_idx] += 1;
                    if self.octos[n_idx] > 9 {
                        neighbors_which_will_flash.push(n_idx);
                    }
                }
            }
            self.octos[idx] = 0;
            already_flashed[idx] = true;
        }
        neighbors_which_will_flash
    }

    fn step(&mut self) -> usize { 
        self.increase_enegry();
        let mut already_flashed: Vec<bool> = vec![false; self.octos.len()];
        let mut carryover_flashes: Vec<usize> = Vec::new();
        for row in 0..10 {
            for col in 0..10 {
                carryover_flashes.append(&mut self.step_cell(row, col, &mut already_flashed));
            }
        }

        while !carryover_flashes.is_empty() {
            let mut next_carryover = vec![];
            for idx in carryover_flashes {
                let (row, col) = OctoBoard::inv_idx(idx);
                next_carryover.append(&mut self.step_cell(row, col, &mut already_flashed));
            }
            carryover_flashes = next_carryover;
        }
        already_flashed.into_iter().filter(|x| *x).count()
    }

    fn print_str(&self) {
        let mut out = String::from(""); 
        for row in 0..10 {
            for col in 0..10 {
                out += self.index((row, col)).unwrap().to_string().as_str();
            }
            out += "\n";
        }
        println!("{}", out);
    }
}



fn main() {
    let mut octoboard = OctoBoard::new(file_to_vec("input/11/a.txt")); 
    for _ in 0..100 {
        octoboard.step();
        octoboard.print_str();
    }
    println!("{}", octoboard.total_flashes);
}
