// const md: i64 = 1000000007;
// impl Solution {
//     pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
//         let mut ans = i64::MIN;
//         solver(&grid, 0, 0, grid.len(), grid[0].len(), 1, &mut ans);
//         if ans < 0 {
//             -1
//         } else { (ans % md).try_into().unwrap() }
//     }
// }

// fn solver(grid: &Vec<Vec<i32>>, i: usize, j: usize, m: usize, n: usize, cur_mx: i64, mx: &mut i64) {
//     if i == m-1 && j == n-1 {
//         let x = *mx;
//         *mx = x.max(cur_mx * grid[i][j] as i64);
//         return;
//     }
//     if i >= m || j >= n { return; }
//     solver(grid, i+1, j, m, n, cur_mx * grid[i][j] as i64, mx);
//     solver(grid, i, j+1, m, n, cur_mx * grid[i][j] as i64, mx);
// }


// const md: i64 = 1000000007;
// impl Solution {
//     pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
//         let (m, n) = (grid.len(), grid[0].len());
//         let mut pos = vec![vec![Some(i64::MIN); n]; m];
//         let mut neg = vec![vec![Some(i64::MAX); n]; m];

//         for i in 0..m {
//             for j in 0..n {
//                 let (cur_p, cur_n) = if i == 0 && j == 0 {
//                     if grid[i][j] < 0 { (None, Some(grid[i][j] as i64)) }
//                     else { (Some(grid[i][j] as i64), None) }
//                 } else if grid[i][j] > 0 {
//                     if pos[i][j].is_some() && neg[i][j].is_some() { (pos[i][j]*grid[i][j]  as i64, neg[i][j]*grid[i][j] as i64) }
//                     else if pos[i][j].is_some() { (pos[i][j]*grid[i][j]  as i64, None) }
//                     else { (None, neg[i][j]*grid[i][j] as i64) }
//                 } else {
//                     (neg[i][j]*grid[i][j] as i64, pos[i][j]*grid[i][j] as i64)
//                 };
//                 pos[i][j] = cur_p;
//                 neg[i][j] = cur_n;
//                 if i < m-1 {
//                     if cur_p.is_some() {pos[i+1][j] = pos[i+1][j].max(cur_p);}
//                     if cur_n.is_some() {neg[i+1][j] = neg[i+1][j].min(cur_n);}
//                 }
//                 if j < n-1 {
//                     if cur_p.is_some() {pos[i][j+1] = pos[i][j+1].max(cur_p);}
//                     if cur_n.is_some() {neg[i][j+1] = neg[i][j+1].min(cur_n);}
//                 }
//         println!("{:?}, {:?}\n\n", pos, neg);
//             }
//         }

//         if pos[m-1][n-1].is_none() {
//             -1
//         } else { (pos[m-1][n-1] % md).try_into().unwrap() }
//     }
// }


impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp:Vec<Vec<(i64,i64)>> = vec![vec![(0,0);grid[0].len()];grid.len()];
        dp[0][0] = (grid[0][0] as i64,grid[0][0] as i64);
        let mut result = dp[0][0].0.max(dp[0][0].1);
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                let mut vals = vec![];
                if col>0 {
                    vals.push(grid[row][col] as i64*dp[row][col-1].0);
                    vals.push(grid[row][col] as i64*dp[row][col-1].1);
                }
                if row>0 {
                    vals.push(grid[row][col] as i64*dp[row-1][col].0);
                    vals.push(grid[row][col] as i64*dp[row-1][col].1);
                }
                if !vals.is_empty() {
                    dp[row][col] = (*vals.iter().min().unwrap(),*vals.iter().max().unwrap());
                    result = dp[row][col].0.max(dp[row][col].1);
                }
            }
        }  
        if result >= 0 {(result % 1000000007) as i32} else {-1}
    }
}