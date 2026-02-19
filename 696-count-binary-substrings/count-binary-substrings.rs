impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut a, mut b, mut ans) = (0, 0, 0);
        if s[0] == '0' as u8 {
            a = 1;
        } else { b = 1; }
        for i in 1..s.len(){
            if s[i] != s[i-1] {
                ans += a.min(b);
                if s[i] == '0' as u8 {
                    a = 0;
                } else { b = 0; }
            }

            if s[i] == '0' as u8 {
                a += 1;
            } else { b += 1; }
        }
        ans += a.min(b);
        ans
    }
}