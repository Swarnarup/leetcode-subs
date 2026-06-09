impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        k as i64 * (nums.iter().fold(i32::MIN, |acc, &x| acc.max(x)) - nums.iter().fold(i32::MAX, |acc, &x| acc.min(x))) as i64
    }
}