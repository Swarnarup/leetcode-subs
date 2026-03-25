impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());

        let mut sm = grid.iter().fold(0 as i64, |acc, arr| {
            acc + arr.iter().fold(0 as i64, |acc, x| acc + *x as i64)
        });

        let mut curr = 0 as i64;
        for arr in &grid {
            curr += arr.iter().fold(0 as i64, |acc, x| acc + *x as i64);
            if curr == (sm - curr) { return true; }
        }

        curr = 0 as i64;
        for j in 0..n {
            let mut tmp = 0;
            for i in 0..m { tmp += grid[i][j] as i64; }
            curr += tmp;
            if curr == (sm - curr) { return true; }
        }
        false
    }
}