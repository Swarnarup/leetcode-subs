impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        nums[n/2]
    }
}