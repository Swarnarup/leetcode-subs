impl Solution {
    pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        // XOR is commt. and asst.
        
        for q in queries {
            let (l, r, k, v) = (q[0], q[1], q[2], q[3]);
            let mut idx = l as usize;
            while idx <= r as usize {
                nums[idx] = ((nums[idx] as i64 * v as i64) % (1000000007)) as i32;
                idx += k as usize;
            }
        }
        // println!("{:?}", nums);
        nums.iter().fold(0, |acc, x| acc ^ *x)
    }
}