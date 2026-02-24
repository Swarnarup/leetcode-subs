use std::collections::HashSet;
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        if s.len() < k as usize { return false; }
        let mut i = 0;
        let mut st: HashSet<i32> = HashSet::new();
        for j in 0..s.len() {
            if j - i + 1 >= k as usize {
                st.insert(build_int(s.as_bytes(), i, j));
                i+=1;
            }
        }
        st.len() == (1<<k)
    }
}


fn build_int(s: &[u8], i: usize, j: usize) -> i32 {
    let mut res = 0;
    for k in i..=j{
        res = res << 1;
        if s[k] == '1' as u8 {
            res = res | 1;
        }
    }
    res
}