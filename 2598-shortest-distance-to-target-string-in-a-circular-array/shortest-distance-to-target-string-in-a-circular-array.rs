// impl Solution {
//     pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
//         let start_index = start_index as usize;
//         let mut ans = i32::MAX;
//         for i in (0..start_index+1).rev() {
//             if words[i] == target {
//                 ans = ans.min((start_index - i) as i32);
//                 break;
//             }
//         }
//         for i in start_index..words.len() {
//             if words[i] == target {
//                 ans = ans.min((i - start_index) as i32);
//                 break;
//             }
//         }

//         ans.min()
//     }
// }


use std::cmp::min;

impl Solution {
    fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len();
        let start = start_index as usize;
        let mut min_dist = usize::MAX;

        for i in 0..n {
            if words[i] == target {
                let right_dist = if i >= start {
                    i - start
                } else {
                    n - start + i
                };

                let left_dist = n - right_dist;

                let local_min = right_dist.min(left_dist);
                min_dist = min_dist.min(local_min);
            }
        }

        if min_dist == usize::MAX {
            -1
        } else {
            min_dist as i32
        }
    }
}