impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let mut v10: Vec<char> = Vec::new();
        let mut v11: Vec<char> = Vec::new();
        for (i, ch) in s1.chars().enumerate(){
            if i&1 == 0 { v10.push(ch); }
            else { v11.push(ch); }
        }
        v10.sort();
        v11.sort();
        let mut v20: Vec<char> = Vec::new();
        let mut v21: Vec<char> = Vec::new();
        for (i, ch) in s2.chars().enumerate(){
            if i&1 == 0 { v20.push(ch); }
            else { v21.push(ch); }
        }
        v20.sort();
        v21.sort();
        v10 == v20 && v11 == v21
    }
}