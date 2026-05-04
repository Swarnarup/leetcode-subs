impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // idea - transpose and reflect
        let (m, n) = (matrix.len(), matrix[0].len());
        for i in 0..m {
            for j in i..n {
                (matrix[i][j], matrix[j][i]) = (matrix[j][i], matrix[i][j]);
            }
        }
        // println!("{:?}", &matrix);
        for i in 0..m {
            for j in 0..n/2 {
                (matrix[i][j], matrix[i][n-1-j]) = (matrix[i][n-1-j], matrix[i][j]);
            }
        }
    }
}