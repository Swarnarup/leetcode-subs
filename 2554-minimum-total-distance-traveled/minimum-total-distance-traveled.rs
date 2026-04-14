// impl Solution {
//     pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        
//     }
// }


impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort_unstable();
        factory.sort_unstable_by_key(|f| f[0]);

        let mut slots: Vec<i32> = Vec::new();
        for f in &factory {
            for _ in 0..f[1] {
                slots.push(f[0]);
            }
        }

        let n = robot.len();
        let m = slots.len();
        const INF: i64 = i64::MAX / 2;

        let mut dp = vec![0i64; m + 1];

        for i in 1..=n {
            let mut new_dp = vec![INF; m + 1];

            for j in 1..=m {
                let skip   = new_dp[j - 1];
                let assign = dp[j - 1].saturating_add(
                    (robot[i - 1] - slots[j - 1]).abs() as i64
                );
                new_dp[j] = skip.min(assign);
            }

            dp = new_dp;
        }

        dp[m]
    }
}