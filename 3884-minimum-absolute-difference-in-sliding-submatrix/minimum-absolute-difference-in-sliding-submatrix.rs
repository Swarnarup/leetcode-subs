// impl Solution {
//     pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        
//     }
// }


use itertools::Itertools;

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        (0..=grid.len() - k as usize)
            .map(|sub_i| {
                (0..=grid[0].len() - k as usize)
                    .map(|sub_j| {
                        grid.iter()
                            .skip(sub_i)
                            .take(k as usize)
                            .flat_map(|row| row.iter().skip(sub_j).take(k as usize))
                            .sorted()
                            .dedup()
                            .tuple_windows()
                            .map(|(&a, &b)| b - a)
                            .min()
                            .unwrap_or(0)
                    })
                    .collect()
            })
            .collect()
    }
}