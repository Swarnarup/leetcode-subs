impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut f = false;
        while n > 0 {
            f = if (n & 1) == 1 { true } else { false };
            n = n >> 1;
            if !(f ^ ((n & 1) == 1)) {
                return false;
            }
        }
        true
    }
}