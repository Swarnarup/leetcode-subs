impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut u = 0;
        for ch in moves.chars() {
            match ch {
                'L' => l+=1,
                'R' => r += 1,
                _ => u += 1
            }
        }
        u + l.max(r) - l.min(r)
    }
}