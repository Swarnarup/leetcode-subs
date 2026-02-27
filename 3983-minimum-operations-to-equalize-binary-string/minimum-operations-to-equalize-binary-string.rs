// impl Solution {
//     pub fn min_operations(s: String, k: i32) -> i32 {
//         let mut ans = 0;
//         let mut arr = s.as_bytes();
//         for sarr in &mut arr.windows(k as usize) {
//             let mut j = 0;
//             let mut flg = 0;
//             if sarr[j] == '0' as u8 {
//                 flg = 1;
//                 while j < sarr.len() {
//                     *sarr[j] = if sarr[j] == '0' as u8 {'1' as u8} else {'0' as u8};
//                     j+=1;
//                 }
//             }
//             ans += flg;
//         }
//         ans
//     }
// }

impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let s_bytes = s.as_bytes();
        let s_len = s_bytes.len() as i32;

        // Scout the battlefield — count zeros
        let mut zero = 0;
        for &b in s_bytes {
            zero += (!b & 1) as i32;
        }

        // Quick victory check
        if zero == 0 { return 0; }

        // Full-length block mission
        if s_len == k {
            return ((if zero == s_len { 1 } else { 0 }) << 1) - 1;
        }

        let base = s_len - k;

        // Odd strike plan — neutralize both zeros and ones
        let mut odd = std::cmp::max(
            (zero + k - 1) / k,
            ((s_len - zero) + base - 1) / base,
        );
        odd += !odd & 1; // Dhruv bitwise trick: force odd

        // Even strike plan — neutralize zeros only
        let mut even = std::cmp::max(
            (zero + k - 1) / k,
            (zero + base - 1) / base,
        );
        even += even & 1; // Dhruv bitwise trick: force even

        let mut res = i32::MAX;

        // Deploy only valid plans
        if (k & 1) == (zero & 1) { res = res.min(odd); }
        if (!zero & 1) == 1 { res = res.min(even); }

        if res == i32::MAX { -1 } else { res }
    }
}