impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let arr: Vec<char> = s.chars().collect();
        let n = arr.len();
        let (mut a_right, mut b_left) = (vec![0; n], vec![0; n]);
        for i in 1..n {
            b_left[i] = if arr[i-1] == 'b' {b_left[i-1]+1} else {b_left[i-1]};
        }
        for i in (0..n-1).rev() {
            a_right[i] = if arr[i+1] == 'a' {a_right[i+1]+1} else {a_right[i+1]};
        }
        // println!("{:?} \n {:?}", a_right, b_left);
        let mut ans = i32::MAX;
        for i in 0..n {
            ans = ans.min(b_left[i]+a_right[i]);
        }
        ans
    }
}