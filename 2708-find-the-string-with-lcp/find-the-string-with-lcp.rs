// impl Solution {
//     pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        
//     }
// }


impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut cand = vec!['a'; n];

        // Update cand based on the LCP conditions
        for r in 0..n {
            for c in r..n {
                if lcp[r][c] == 0 && cand[c] == cand[r] {
                    cand[c] = (cand[c] as u8 + 1) as char; // Bump character
                }
            }
        }

        // Check if any character exceeds 'z'
        if cand.iter().any(|&ch| ch > 'z') {
            return String::new();
        }

        // Use DP to build the would-be LCP matrix
        let mut would = vec![vec![0; n + 1]; n + 1];

        for r in (0..n).rev() {
            for c in (0..n).rev() {
                if cand[r] == cand[c] {
                    would[r][c] = 1 + would[r + 1][c + 1];
                }
            }
        }

        // Compare the constructed LCP matrix with the given one
        for r in 0..n {
            for c in 0..n {
                if would[r][c] != lcp[r][c] {
                    return String::new();
                }
            }
        }

        cand.iter().collect() // Convert the vector of chars into a string
    }
}