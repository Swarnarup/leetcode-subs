impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut flg: bool = false;
        let mut x = '0';
        for y in s.chars() {
            if flg && (x == '0' && y == '1') { return false; }
            if x == '1' && y == '0' {
                flg = true;
            }
            x = y;
        }
        true
    }
}