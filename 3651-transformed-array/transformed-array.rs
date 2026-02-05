impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        let mut ans = vec![0; n as usize];
        for (i, v) in nums.iter().enumerate(){
            ans[i] = nums[(((nums[i] % n) + i as i32 + n) % n) as usize];
        }

        ans
    }
}