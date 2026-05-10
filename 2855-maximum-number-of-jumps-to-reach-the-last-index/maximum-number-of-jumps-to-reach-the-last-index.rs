impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![-1; nums.len()];
        dp[0] = 0;
        for i in 0..nums.len() {
            if (dp[i] == -1) { continue; }
            for j in i+1..nums.len() {
                // println!("{} {} {} {}", i, j, nums[i] - nums[j], (nums[i] - nums[j]).abs());
                if (nums[i] - nums[j]).abs() <= target {
                    dp[j] = dp[j].max(dp[i] + 1);
                }
            }
            // dp[i] = tmp;
        }
        // println!("{:?}", dp);
        dp[nums.len() - 1]
    }
}