// impl Solution {
//     pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
//         let mut g = vec![vec![true; grid[0].len()]; grid.len()];
//         let mut l = vec![vec![true; grid[0].len()]; grid.len()];
//         let mv = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
//         for i in 0..grid.len() {
//             for j in 0..grid[0].len() {
//                 if !g[i][j] {
//                     g[i][j] = true;
//                     if dfs(i as i32, j as i32, &mv, &grid, &mut g, &mut l) {
//                         return true;
//                     }
//                 }
//             }
//         }
//         return false;
//     }
// }

// fn dfs(i: i32, j: i32, mv: &Vec<(i32, i32)>, grid: &Vec<Vec<char>>, g_visit: &mut Vec<Vec<bool>>, l_visit: &mut Vec<Vec<bool>>) -> bool {
//     l_visit[i as usize][j as usize] = true;
//     for (di, dj) in mv {
//         let (ii, jj) = (i + *di, j + *dj);
//         if ii < 0 || ii >= grid.len() as i32 || jj < 0 || jj >= grid[0].len() as i32 || g_visit[ii as usize][jj as usize] || grid[ii as usize][jj as usize] != grid[i as usize][j as usize] {
//             continue;
//         }
//         let (ii, jj) = (ii as usize, jj as usize);
//         if l_visit[ii][jj] {
//             return true;
//         }
//         g_visit[ii][jj] = true;
//         if dfs(ii as i32, jj as i32, mv, grid, g_visit, l_visit) {
//             return true;
//         }
//     }
//     l_visit[i as usize][j as usize] = false;
//     false
// }

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let n = grid.len();
        let m = grid[0].len();
        
        let p = [(1,0),(0,1),(-1,0),(0,-1)];
        let mut seen = vec![vec![false;m];n];
        for i in 0..n {
            for j in 0..m {
                if seen[i][j] { continue }

                let cc = grid[i][j];
                let mut stack = vec![(i as isize, j as isize, 10000,10000)];

                while let Some((ci,cj,li,lj)) = stack.pop() {
                    seen[ci as usize][cj as usize] = true;
                    for &(ai, aj) in &p {
                        let ni = ci + ai;
                        let nj = cj + aj;
                        if ni == li && nj == lj { continue }
                        if ni < 0 || nj < 0 || n as isize <= ni || m as isize <= nj { continue }
                        let nui = ni as usize;
                        let nuj = nj as usize;          

                        if grid[nui][nuj] == cc {
                            if seen[nui][nuj] {
                                return true
                            }
                            stack.push((ni,nj, ci, cj));  
                        }
                    }
                }
            }
        }
        false
    }
}