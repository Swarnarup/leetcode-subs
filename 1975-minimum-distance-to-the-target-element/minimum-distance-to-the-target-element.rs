

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let start = start as usize;
        let mut ans = i32::MAX;
        for i in (0..start+1).rev() {
            if nums[i] == target {
                ans = ans.min((start - i) as i32);
                break;
            }
        }
        for i in start..nums.len() {
            if nums[i] == target {
                ans = ans.min((i - start) as i32);
                break;
            }
        }

        ans
    }
}