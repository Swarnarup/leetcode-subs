impl Solution {
    // pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        
    // }
    pub fn max_total_value(a: Vec<i32>, k: i32) -> i64 {
    let n = a.len();
    let mut x = vec![a; 17];
    let mut y = x.clone();

    for j in 0..16 {
        let p = 1 << j;
        if p <= n / 2 {
            for i in 0..=n - 2*p {
                x[j + 1][i] = x[j][i].min(x[j][i + p]);
                y[j + 1][i] = y[j][i].max(y[j][i + p])
            }
        }
    }
    let q = |l: usize, r: usize| {
        let j = (r - l).ilog2() as usize;
        let p = 1 << j;
        ((y[j][l].max(y[j][r - p]) 
        - x[j][l].min(x[j][r - p])) as i64, l, r)
    };

    let mut h = std::collections::BinaryHeap::new();
    for l in 0..n { h.push(q(l, n)) }

    (0..k).map(|_| {
        let (v, l, r) = h.pop().unwrap();
        if r - l > 1 { h.push(q(l, r - 1)) }
        v
    }).sum()
}
}