impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut ans = 0;
        for i in 1..n+1 {
            ans = concat(ans, i);
        }
        ans
    }
}

fn concat(x: i32, y: i32) -> i32{
    let offset = 32 - y.leading_zeros();
    let mut x = x as i64;
    x = (x << offset) | y as i64;
    x = x % 1000000007;
    x as i32
}