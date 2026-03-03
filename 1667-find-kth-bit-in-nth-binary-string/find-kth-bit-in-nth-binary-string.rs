impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mid = 1 << (n-1);
        if n == 1 { return '0'; }
        if mid == k { return '1'; }
        if k < mid {
            return Self::find_kth_bit(n-1, k);
        }
        let ch = Self::find_kth_bit(n-1, 2*mid-k);
        if ch == '0' { return '1'; }
        '0'
    }
}