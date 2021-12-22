use advent_of_code_2021::file_to_vec;
struct HeightMap {
    heights: Vec<u32>,
    rows: usize,
    cols: usize
}

impl HeightMap {
    fn new(lines: Vec<String>) -> Self {
        let cols = lines[0].chars().count();
        let heights: Vec<u32> = lines.iter().flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap())).collect();
        let rows = heights.len() / cols;
        HeightMap { heights, rows, cols}
    }

    fn idx(&self, row: i32, col: i32) -> Option<usize> {
        if row < 0 || col < 0 || row as usize >= self.rows || col as usize >= self.cols {
            None
        } else {
            Some((row as usize * self.cols  + col as usize) as usize)
        }
    }
     
    fn neighbor_idxs(&self, row: i32, col: i32) -> Vec<usize> {
        [
            self.idx(row-1, col),
            self.idx(row, col-1),
            self.idx(row, col +1), 
            self.idx(row+1, col)
        ]
        .iter()
        .filter_map(|x| *x)
        .collect()
    }

    fn is_minima(&self, row: usize, col: usize) -> bool {
        let idx = self.idx(row as i32, col as i32).unwrap();
        self.neighbor_idxs(row as i32, col as i32).iter().all(|n_idx| self.heights[*n_idx] > self.heights[idx])
    }
    
    fn risk_level(&self, row: usize, col: usize) -> u32 {
        let idx = self.idx(row as i32, col as i32).unwrap();
        self.heights[idx] + 1
    }
}

fn main() {
    let heightmap = HeightMap::new(file_to_vec("input/9/a.txt"));
    let mut total_risk = 0;
    for row in 0..heightmap.rows {
        for col in 0..heightmap.cols {
            if heightmap.is_minima(row, col) { 
                total_risk += heightmap.risk_level(row, col);
            }
        }
    }

    println!("Total risk {}", total_risk);
}
