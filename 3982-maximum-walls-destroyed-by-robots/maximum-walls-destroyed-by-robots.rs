// impl Solution {
//     pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        
//     }
// }

use std::collections::HashSet;

impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        if robots.is_empty() {
            return 0;
        }

        // 1. Group robots and distance, then sort by position
        let mut p_d: Vec<(i32, i32)> = robots.into_iter().zip(distance.into_iter()).collect();
        p_d.sort_unstable_by_key(|&(p, _)| p);

        // 2. Extract unique positions and best firing actions
        let mut unique_positions = Vec::new();
        let mut actions = Vec::new(); // Store pairs of (Left_Distance, Right_Distance)

        let mut i = 0;
        let n = p_d.len();
        while i < n {
            let pos = p_d[i].0;
            let mut d1 = p_d[i].1;
            let mut d2 = -1; // -1 denotes no second robot at this position

            let mut j = i + 1;
            while j < n && p_d[j].0 == pos {
                let d = p_d[j].1;
                if d > d1 {
                    d2 = d1;
                    d1 = d;
                } else if d > d2 {
                    d2 = d;
                }
                j += 1;
            }

            unique_positions.push(pos);
            if d2 == -1 {
                actions.push(vec![(d1, 0), (0, d1)]);
            } else {
                actions.push(vec![(d1, d2), (d2, d1)]);
            }
            i = j; // Advance to the next unique position
        }

        // 3. Extract walls located exactly on robots
        let robot_positions_set: HashSet<i32> = unique_positions.iter().copied().collect();
        let mut base_destroyed = 0;
        let mut remaining_walls = Vec::new();

        for w in walls {
            if robot_positions_set.contains(&w) {
                base_destroyed += 1;
            } else {
                remaining_walls.push(w);
            }
        }
        remaining_walls.sort_unstable();

        // Helper closure to count walls in [l, r] using binary search (partition_point)
        // Uses i64 to prevent any coordinate overflow panics
        let count_walls = |l: i64, r: i64| -> i32 {
            if l > r {
                return 0;
            }
            let right_idx = remaining_walls.partition_point(|&w| (w as i64) <= r);
            let left_idx = remaining_walls.partition_point(|&w| (w as i64) < l);
            (right_idx - left_idx) as i32
        };

        let k = unique_positions.len();
        let mut prev_dp = vec![0; actions[0].len()];

        // 4. Base Case: Walls destroyed to the left of the very first robot group
        for j in 0..actions[0].len() {
            let l_dist = actions[0][j].0 as i64;
            let x_0 = unique_positions[0] as i64;
            prev_dp[j] = count_walls(x_0 - l_dist, x_0 - 1);
        }

        // 5. DP Transitions for segments between robots
        for i in 1..k {
            let mut curr_dp = vec![0; actions[i].len()];
            let x_prev = unique_positions[i - 1] as i64;
            let x_curr = unique_positions[i] as i64;

            for j_curr in 0..actions[i].len() {
                let mut max_val = -1;
                let l_dist_curr = actions[i][j_curr].0 as i64;

                for j_prev in 0..actions[i - 1].len() {
                    let r_dist_prev = actions[i - 1][j_prev].1 as i64;

                    // Bounds bounded by neighboring robots
                    let r_reach = std::cmp::min(x_prev + r_dist_prev, x_curr - 1);
                    let l_reach = std::cmp::max(x_curr - l_dist_curr, x_prev + 1);

                    let walls_r = count_walls(x_prev + 1, r_reach);
                    let walls_l = count_walls(l_reach, x_curr - 1);

                    let intersection = if l_reach <= r_reach {
                        count_walls(l_reach, r_reach)
                    } else {
                        0
                    };

                    let seg_destroyed = walls_r + walls_l - intersection;
                    max_val = std::cmp::max(max_val, prev_dp[j_prev] + seg_destroyed);
                }
                curr_dp[j_curr] = max_val;
            }
            prev_dp = curr_dp; // Move to the next sequence
        }

        // 6. Finalize with walls to the right of the very last robot group
        let mut max_additional = 0;
        let x_last = unique_positions[k - 1] as i64;

        for j in 0..actions[k - 1].len() {
            let r_dist = actions[k - 1][j].1 as i64;
            let total_for_action = prev_dp[j] + count_walls(x_last + 1, x_last + r_dist);
            max_additional = std::cmp::max(max_additional, total_for_action);
        }

        base_destroyed + max_additional
    }
}