
// just need to catch the max depth..
/*
[[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]               -> 32
[[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8]]         -> 64
[[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9]]   -> 128
*/

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len();
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n+2];
        for e in edges {
            adj[e[0] as usize].push(e[1] as usize);
            adj[e[1] as usize].push(e[0] as usize);
        }
        let mut mxL = 0;
        let mut mxN = 0;
        dfs(1, 0, &adj, 0, &mut mxL, &mut mxN);
        binary_exp(mxL - 1, 1000000007)
    }
}

fn dfs(node: usize, par: usize, adj: &Vec<Vec<usize>>, len: i32, maxLen: &mut i32, maxNode: &mut usize) {
    if len > *maxLen {
        *maxLen = len;
        *maxNode = node;
    }
    for &next in &adj[node] {
        if next == par { continue; }
        dfs(next, node, adj, len + 1, maxLen, maxNode);
    }
}

// calculates 2^n % mod
fn binary_exp(n: i32, m: i64) -> i32 {
    // println!("{}", n);
    if n == 1 {
        return 2;
    }
    if n <= 0 {
        return 1;
    }
    let half = binary_exp(n>>1, m);
    let full: i64 = ((half as i64) * (half as i64)) % m;
    // println!("{} {}", half, full);
    if n & 1 == 0 {
        return full as i32
    }
    return (((2 as i64) * full) % m) as i32;
} 