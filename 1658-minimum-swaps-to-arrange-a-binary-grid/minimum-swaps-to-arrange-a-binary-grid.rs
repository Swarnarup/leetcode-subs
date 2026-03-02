// impl Solution {
//     pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        
//     }
// }

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut values = vec![0; n];

        for i in 0..n {
            let mut val = 0;
            for j in (1..n).rev() {
                if grid[i][j] == 1 {
                    val = j as i32;
                    break;
                }
            }
            values[i] = val;
        }
        let mut steps = 0;

        for i in 0..n {
            let mut j = i+1;
            while i < values[i] as usize {
                if j == n {
                    return -1;
                }
                values.swap(i, j);
                j += 1;
                steps += 1;
            }
        }
        steps
    }
}
