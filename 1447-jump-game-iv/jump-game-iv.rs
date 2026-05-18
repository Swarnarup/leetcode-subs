// use std::collections::HashMap;
// impl Solution {
//     pub fn min_jumps(arr: Vec<i32>) -> i32 {
//         let mut dp = vec![i32::MAX; arr.len()];
//         let mut mp: HashMap<i32, i32> = HashMap::new();
//         mp.insert(arr[arr.len() - 1], 0);
//         dp[arr.len() - 1] = 0;
        
//         for i in (0..arr.len()-1).rev() {
//             if let Some(x) = mp.get_mut(&arr[i]) {
//                 dp[i] = *x + 1;
//                 for j in (i+1)..(i+(*x as usize)) {
//                     dp[i] = dp[i].min(dp[j] + (j-i)as i32);
//                 }
//                 *x = dp[i].min(*x);
//                 for j in (i+1)..arr.len()-1 {
//                     if dp[j] > dp[i] + (j-i)as i32 {
//                         dp[j] = dp[i] + (j-i) as i32;
//                         if let Some(y) = mp.get_mut(&arr[j]) {
//                             *y = dp[j].min(*y);
//                         }
//                     }
//                 }
//             }
//             else {
//                 let mut tmp = i32::MAX;
//                 for j in (i+1)..arr.len() {
//                     tmp = tmp.min(dp[j] + (j-i)as i32);
//                 }
//                 dp[i] = tmp;
//                 mp.insert(arr[i], dp[i]);
//             }
//         }
//         // println!("{:?}", dp);
//         dp[0]
//     }
// }



use std::collections::HashMap;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut hash = HashMap::new();
        for (i, &x) in arr.iter().enumerate() {
            hash.entry(x).or_insert(Vec::new()).push(i);
        }
        let mut visited = vec![false; arr.len()];
        visited[0] = true;
        
        let mut curr = vec![0];
        let mut next = Vec::new();
        for step in 0.. {
            for i in curr.drain(..) {
                if i == arr.len() - 1 {
                    return step;
                }
                for j in hash.remove(&arr[i])
                            .unwrap_or(Vec::new())
                            .into_iter()
                            .chain(vec![i.saturating_sub(1), i + 1].into_iter()) {
                    if !visited[j] {
                        next.push(j);
                        visited[j] = true;
                    }
                }
            }
            std::mem::swap(&mut curr, &mut next);
        }
        unreachable!()
    }
}