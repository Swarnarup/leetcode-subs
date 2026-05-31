impl Solution {
    pub fn asteroids_destroyed(mut mass: i32, mut asteroids: Vec<i32>) -> bool {
        asteroids.sort();
        !asteroids.iter().any(|&ast| {
            if mass < ast {
                return true;
            }
            if mass as u64 + ast as u64 > i32::MAX as u64 {
                mass = i32::MAX;
            }
            else {mass += ast;}
            false
        })
    }
}