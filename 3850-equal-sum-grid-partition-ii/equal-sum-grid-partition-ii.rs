// impl Solution {
//     pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
//         let (m, n) = (grid.len(), grid[0].len());
//         let mut nums = vec![(0, 0, 0); (m*n)];
//         for i in 0..m {
//             for j in 0..n {
//                 nums[(i+1)*j] = (grid[i][j] as i64, i, j);
//             }
//         }
//         nums.sort();
//         let mut sm = grid.iter().fold(0 as i64, |acc, arr| {
//             acc + arr.iter().fold(0 as i64, |acc, x| acc + *x as i64)
//         });

//         let mut curr = 0 as i64;
//         let mut ii = 0;
//         for arr in &grid {
//             curr += arr.iter().fold(0 as i64, |acc, x| acc + *x as i64);
//             let next = sm - curr;
//             if curr == next { return true; }
//             else {
//                 let diff = curr - next;
//                 if diff < 0 {
//                     if let Some((num, i, j)) = bs_left(&nums, -diff) {
//                         println!("{} {} {} {}", 1, i, j, diff);
//                         if num == -diff && i > ii {
//                             return true;
//                         }
//                     }
//                 }
//                 else {
//                     if let Some((num, i, j)) = bs_right(&nums, diff) {
//                         println!("{} {} {} {}", 2, i, j, diff);
//                         if num == diff && i <= ii {
//                             return true;
//                         }
//                     }
//                 }
//             }
//             ii+=1;
//         }

//         curr = 0 as i64;
//         for jj in 0..n {
//             let mut tmp = 0;
//             for i in 0..m { tmp += grid[i][jj] as i64; }
//             curr += tmp;
//             let next = sm - curr;
//             if curr == next { return true; }
//             else {
//                 let diff = curr - next;
//                 if diff < 0 {
//                     if let Some((num, i, j)) = bs_left(&nums, -diff) {
//                         println!("{} {} {} {}", 3, i, j, diff);
//                         if num == -diff && j > jj {
//                             return true;
//                         }
//                     }
//                 }
//                 else {
//                     if let Some((num, i, j)) = bs_right(&nums, diff) {
//                         println!("{} {} {} {}", 4, i, j, diff);
//                         if num == diff && j <= jj {
//                             return true;
//                         }
//                     }
//                 }
//                 println!("-->{} {} {}", curr, next, diff);
//             }
//         }
//         false
//     }
// }

// fn bs_left(arr: &Vec<(i64,usize, usize)>, target: i64) -> Option<(i64, usize, usize)> {
//     let mut ans = None;
//     let (mut i, mut j, mut mid) = (0, arr.len() - 1, 0);
//     while i as i32 <= j as i32 {
//         mid = (i+j)/2;
//         if arr[mid].0 >= target {
//             ans = Some(arr[mid]);
//             j = mid - 1;
//         } else {
//             i = mid + 1;
//         }
//     }
//     ans
// } 
// fn bs_right(arr: &Vec<(i64,usize, usize)>, target: i64) -> Option<(i64,usize, usize)> {
//     let mut ans = None;
//     let (mut i, mut j, mut mid) = (0, arr.len()-1, 0);
//     while i as i32 <= j as i32 {
//         mid = (i+j)/2;
//         if arr[mid].0 <= target {
//             ans = Some(arr[mid]);
//             i = mid + 1;
//         } else {
//             j = mid - 1;
//         }
//     }
//     ans
// } 

use std::collections::HashMap;
impl Solution {
    pub fn can_partition_grid(g: Vec<Vec<i32>>) -> bool {
        let l: Vec<Vec<_>> = g.iter().map(|r| r.iter().map(|&v| v as i64).collect()).collect();
        let t: i64 = l.iter().flatten().sum();
        let x: Vec<Vec<_>> = (0..l[0].len()).map(|c| l.iter().map(|r| r[c]).collect()).collect();
        let (mut lr, mut xr) = (l.clone(), x.clone()); lr.reverse(); xr.reverse();
        let f = |m: &Vec<Vec<i64>>| {
            let (r, c, mut p, mut map) = (m.len() - 1, m[0].len() - 1, 0, HashMap::new());
            (0..=r).any(|y| {
                m[y].iter().enumerate().any(|(x, &v)| {
                    let i = map.get(&((t - v) / 2)); p += v;
                    (t - v) % 2 == 0 && i.map_or(false, |&i| if c < 1 { y == i + 1 || y == r } else { i < r - 1 || x % c == 0 })
                }) || y < r && { map.insert(p, y); p * 2 == t }
            })
        };
        [&l, &lr, &x, &xr].into_iter().any(|m| f(m))
    }
}