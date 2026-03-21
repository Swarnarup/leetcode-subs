impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        for j in y as usize..(y + k) as usize {
            let mut i = x as usize;
            let mut ii = (x + k) as usize - 1;
            while i < ii {
                (grid[i][j], grid[ii][j]) = (grid[ii][j], grid[i][j]);
                i += 1;
                ii -= 1;
            }
        }
        grid
    }
}