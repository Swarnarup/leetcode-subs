impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut ans = 0;
        for i in 0..32 {
            if n & (1 << (31 - i)) > 0 {
                ans = ans | (1 << i);
            }
        }
        ans
    }
}