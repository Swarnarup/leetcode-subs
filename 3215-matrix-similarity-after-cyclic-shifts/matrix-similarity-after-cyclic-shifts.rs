impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let mut cln = mat.clone();
        let n = mat[0].len() as i32;
        let m = mat.len();

        for i in 0..m {
            if i & 2 == 0 {
                for j in 0..n as usize {
                    let new_j = ((j as i32) - (k%n) + n) % n;
                    let new_j = new_j as usize;
                    cln[i][new_j] = mat[i][j];
                }
            }
            else {
                for j in 0..n as usize {
                    let new_j = ((j as i32) + k) % n;
                    let new_j = new_j as usize;
                    cln[i][new_j] = mat[i][j];
                }
            }
        }
        cln == mat
    }
}