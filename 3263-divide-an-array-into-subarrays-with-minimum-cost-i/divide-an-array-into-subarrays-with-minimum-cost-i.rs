impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let (mut mn, mut mn2) = (i32::MAX, i32::MAX);
        for idx in 1..nums.len(){
            let i = nums[idx];
            if i < mn{
                mn2 = mn;
                mn = i;
            } else if i < mn2 {
                mn2 = i;
            }
        }
        nums[0]+mn+mn2
    }
}