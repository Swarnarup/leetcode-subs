// impl Solution {
//     pub fn make_largest_special(s: String) -> String {
        
//     }
// }


impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut specials = Vec::new();
        let mut count = 0;
        let mut start = 0;
        
        for i in 0..n {
            count += if bytes[i] == b'1' { 1 } else { -1 };
            
            if count == 0 {
                // Found top-level special substring [start..=i]
                // Recursively optimize its inner part [start+1..i-1]
                let inner = Self::make_largest_special(s[start + 1..i].to_string());
                specials.push(format!("1{}0", inner));
                start = i + 1;
            }
        }
        
        // Sort descending for lexicographically largest result
        specials.sort_unstable_by(|a, b| b.cmp(a));
        specials.concat()
    }
}