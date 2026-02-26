impl Solution {
    pub fn num_steps(s: String) -> i32 {
        // iterate from back
        // if no carry -> 
            // 0 -> op+=1 carry = 0
            // 1 -> op+=2 carry = 1
        // if carry ->
            // 0 -> op += 2
            // 1 -> op += 1
        let sz = s.as_bytes().len();
        let mut carry = 0;
        let mut ans = 0;
        for ch in s.as_bytes().iter().rev().take(sz - 1) {
            if carry == 0 {
                ans += if *ch == '0' as u8 {1} else {carry=1; 2};
            } else {
                ans += if *ch == '0' as u8 {2} else {1};
            }
        }
        ans + carry
    }
}
