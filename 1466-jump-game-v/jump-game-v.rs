impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let n = arr.len();
        let mut dp = vec![-1; n];
        let mut ans = 0;
        for i in 0..n {
            if dp[i] == -1 {
                ans = ans.max(solver(i, &arr, &mut dp, d as usize));
            }
        }
        // println!("{:?}", dp);
        ans
    }
}

fn solver(i: usize, arr: &Vec<i32>, dp: &mut Vec<i32>, d: usize) -> i32 {
    if dp[i] != -1 {
        return dp[i];
    }
    let n = arr.len();
    let mut tmp = 0;
    let mut j = (i as i32 - 1) as usize;
    while j as i32 >= 0.max((i as i32 - d as i32)) && arr[j] < arr[i] {
        tmp = tmp.max(solver(j, arr, dp, d));
        j -= 1;
    }
    j = i+1;
    while j <= (n-1).min(i+d) && arr[j] < arr[i] {
        tmp = tmp.max(solver(j, arr, dp, d));
        j += 1;
    }
    dp[i] = tmp + 1;
    dp[i]
}