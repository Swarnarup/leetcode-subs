impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        (n - rev(n)).abs()
    }
}

fn rev(mut i: i32) -> i32 {
    let mut ans = 0;
    while i > 0 {
        ans *= 10;
        ans += i % 10;
        i = i/10;
    }
    ans
}