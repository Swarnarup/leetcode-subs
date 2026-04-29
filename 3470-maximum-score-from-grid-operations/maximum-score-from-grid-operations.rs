// impl Solution {
//     pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        
//     }
// }

impl Solution {
    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        let n=grid.len();
        if n==1{
            return 0;
        }

        let mut prefSum=Vec::new();
        for i in 0..n{
            let mut tmp=vec![0];
            for j in 0..n{
                let b=tmp[tmp.len()-1];
                tmp.push(b+grid[j][i] as i64);
            }
            prefSum.push(tmp);
        }
        let mut prev=vec![vec![0; n+1]; n+1];
        for a in 0..=n{
            for b in 0..=n{
                prev[b][a]=if a>b{prefSum[1][a]-prefSum[1][b]}else{prefSum[0][b]-prefSum[0][a]};
            }
        }
        for j in 2..n{
            let mut dp=vec![vec![0; n+1]; n+1];
            for b in 0..=n{
                for c in 0..=n{
                    let mut m=0;
                    for a in 0..=n{
                        let tmp=prev[b][a]+if c>a&&c>b{
                            prefSum[j-1][c]-prefSum[j-1][a.max(b)]
                        }else if c<b{
                            prefSum[j][b]-prefSum[j][c]
                        }else{0};
                        m=m.max(tmp);
                    }
                    dp[c][b]=m;
                }
            }
            prev=dp;
        }
        *prev.iter().map(|v|v.into_iter().max().unwrap()).max().unwrap()
    }
}