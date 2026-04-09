impl Solution {
    // pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    //     // XOR is commt. and asst.
        
    //     for q in queries {
    //         let (l, r, k, v) = (q[0], q[1], q[2], q[3]);
    //         let mut idx = l as usize;
    //         while idx <= r as usize {
    //             nums[idx] = ((nums[idx] as i64 * v as i64) % (1000000007)) as i32;
    //             idx += k as usize;
    //         }
    //     }
    //     // println!("{:?}", nums);
    //     nums.iter().fold(0, |acc, x| acc ^ *x)
    // }
    // 322ms
    pub fn xor_after_queries(mut n: Vec<i32>, q: Vec<Vec<i32>>) -> i32 {
        const M: i64 = 1000000007; let z = n.len(); let mut c = vec![vec![1; z + 1]; 40];
        fn p(a: i64, b: i64) -> i64 { if b < 1 { 1 } else { p(a * a % M, b / 2) * (if b % 2 > 0 { a } else { 1 }) % M } }
        for u in q {
            let (l, r, k, v) = (u[0] as usize, u[1] as usize, u[2] as usize, u[3] as i64);
            if k < 40 { c[k][l] = c[k][l] * v % M; let x = r - (r - l) % k + k; if x < z { c[k][x] = c[k][x] * p(v, M - 2) % M } }
            else { for i in (l..=r).step_by(k) { n[i] = (n[i] as i64 * v % M) as i32 } }
        }
        for k in 1..40 { for i in 0..z {
            if i >= k { c[k][i] = c[k][i] * c[k][i - k] % M }
            n[i] = (n[i] as i64 * c[k][i] % M) as i32;
        }}
        n.into_iter().fold(0, |a, b| a ^ b)
    }
}