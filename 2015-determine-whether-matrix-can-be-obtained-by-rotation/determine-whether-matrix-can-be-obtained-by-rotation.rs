impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        mat == target || rotate(&mat) == target || rotate(&rotate(&mat)) == target || rotate(&rotate(&rotate(&mat))) == target
    }
}

fn rotate(mat: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = mat.len();
    let mut new_mat = mat.clone();
    for i in 0..n {
        for j in 0..n {
            new_mat[j][n-i-1] = mat[i][j];
        }
    }
    new_mat
}