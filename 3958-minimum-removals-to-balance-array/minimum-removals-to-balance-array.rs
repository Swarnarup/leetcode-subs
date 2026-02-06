// impl Solution {
//     pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
//         nums.sort();
//         let mut ans = 0;
//         for i in (0..nums.len()).rev(){
//             // println!("{} {}", i, nums[i]);
//             if nums[0]*k >= nums[i] { break; }
//             ans+=1;
//         }
//         ans
//     }
// }


impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut i = 0;
        let mut max_len = 0;
        let k_long = k as i64;
        
        for j in 0..nums.len() {
            while (nums[j] as i64) > (nums[i] as i64) * k_long {
                i += 1;
            }
            max_len = max_len.max(j - i + 1);
        }
        
        (nums.len() - max_len) as i32
    }
}