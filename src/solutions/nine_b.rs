use advent_of_code_2021::file_to_vec;
use std::collections::VecDeque;

// I tried to store the board as a vec and just write methods to convert row,col to idx and the
// inverse, but I feel like that made the code harder to read. I tried to stick with the convention
// that operations which accept a usize accept only valid indexes, (similar for return) and
// operations which accept an i32 accept a potentially invalid index (i.e. if I have row = 0, I can
// pass row-1 and it should give the correct result, it doesn't make the code very clear though)
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
            Some(row as usize * self.cols  + col as usize)
        }
    }

    fn inv_idx(&self, idx: usize) -> (usize, usize) {
        let row = idx / self.cols;
        let col = idx % self.cols;
        (row, col)
    }

    fn neighbor_inv_idxs(&self, idx: usize) -> Vec<usize> {
        let (row, col) = self.inv_idx(idx);
        self.neighbor_idxs(row as i32, col as i32)
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


    fn first_unvisited(&self, visited: &Vec<bool>) -> Option<usize> {
        visited.iter().enumerate().filter(|x| !*x.1).map(|x| x.0).next()
    }

    fn connected_components(&self) -> Vec<Vec<usize>> {
        let mut components = Vec::new(); 
        let mut visited = vec![false; self.heights.len()];

        //mark 9s as visited, so they're not a special case
        for i in 0..self.heights.len() {
            if self.heights[i] == 9 {
                visited[i] = true;
            }
        }

        while let Some(start) = self.first_unvisited(&visited) {
            let mut curr_component: Vec<usize> = Vec::new();
            let mut queue: VecDeque<usize> = VecDeque::new();
            queue.push_back(start);
            curr_component.push(start);
            visited[start] = true;
            while !queue.is_empty() {
                let curr_idx = queue.pop_front().unwrap();
                for n_idx in self.neighbor_inv_idxs(curr_idx) {
                    if !visited[n_idx] {
                        queue.push_back(n_idx);
                        curr_component.push(n_idx);
                    }
                    visited[n_idx] = true;
                }
            }
            components.push(curr_component);
        }

        components.sort_by(|l, r| { l.len().partial_cmp(&r.len()).unwrap().reverse()});
        components
    }
}

fn main() {
    let heightmap = HeightMap::new(file_to_vec("input/9/a.txt"));
    println!("{:?}", heightmap.connected_components().iter().take(3).map(|c| c.len()).product::<usize>());
}
