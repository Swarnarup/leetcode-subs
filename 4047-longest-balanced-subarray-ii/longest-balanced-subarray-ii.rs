// impl Solution {
//     pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        
//     }
// }


// use std::collections::HashSet;

// impl Solution {
//     pub fn longest_balanced(nums: Vec<i32>) -> i32 {
//         let (mut ev, mut od) = (HashSet::new(), HashSet::new());
//         let mut ans = 0;

//         for j in 0..nums.len() {
//             (ev, od) = (HashSet::new(), HashSet::new());
//             for i in j..nums.len() {
//                 if (nums[i] & 1) == 0 {
//                     ev.insert(nums[i]);
//                 }
//                 else { od.insert(nums[i]); }
//                 if ev.len() == od.len() {
//                     ans = ans.max(i as i32 - j as i32 + 1);
//                 }
//             }
//         }

//         ans
//     }
// }


impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mx = *nums.iter().max().unwrap();
        let mut ans = 0usize;
        let mut count = vec![0; mx as usize + 1];
        let mut last = vec![n; mx as usize + 1];
        let mut next = vec![n; n];
        let mut uniq = [0; 2];

        for (i, &x) in nums.iter().enumerate() {
            Self::update(x, 1, &mut count, &mut uniq);
            if last[x as usize] < n {
                next[last[x as usize]] = i;
            }
            last[x as usize] = i;
        }

        let mut j = n;
        for i in 0..n - 1 {
            if n - i <= ans {
                break;
            }

            while j - i > ans {
                if uniq[0] == uniq[1] {
                    ans = j - i;
                    break;
                }
                j -= 1;
                Self::update(nums[j], -1, &mut count, &mut uniq);
            }

            while j < next[i] {
                Self::update(nums[j], 1, &mut count, &mut uniq);
                j += 1;
            }
            
            Self::update(nums[i], -1, &mut count, &mut uniq);
        }
        ans as i32
    }

    fn update(x: i32, val: i32,
    count: &mut Vec<i32>, uniq: &mut [i32; 2]) {
        let xu = x as usize;
        count[xu] += val;
        if count[xu] == 0 && val < 0 {
            uniq[xu & 1] -= 1;
        }
        else if count[xu] == val && val > 0 {
            uniq[xu & 1] += 1;
        }
    }
}