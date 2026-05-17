impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut dp = vec![-1; arr.len()];
        solver(start as usize, &arr, &mut dp)
    }
}

fn solver(i: usize, arr: &Vec<i32>, dp: &mut Vec<i32>) -> bool {
    if arr[i] == 0 {
        return true;
    }
    if dp[i] != -1 {
        return dp[i] != 0;
    }
    dp[i] = 0;
    let mut tmp = false;
    if (i as i32 + arr[i]) < arr.len() as i32 && dp[i+(arr[i] as usize)] != 0 {
        tmp |= solver(i+(arr[i] as usize), arr, dp);
    }
    if (i as i32 - arr[i]) >= 0 && dp[i-(arr[i] as usize)] != 0 {
        tmp |= solver(i-(arr[i] as usize), arr, dp);
    }
    dp[i] = tmp as i32;
    tmp
}