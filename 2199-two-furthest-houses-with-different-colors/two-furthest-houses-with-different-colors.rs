impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        if colors[0] != colors[n-1] {
            return (n-1) as i32;
        }
        let mut first = {
            let mut res = n-1;
            for i in 1..n {
                if colors[i] != colors[i-1] {
                    res = i;
                    break;
                }
            }
            res
        };

        let mut last = {
            let mut res = 0;
            for i in (1..n).rev() {
                if colors[i] != colors[i-1] {
                    res = i-1;
                    break;
                }
            }
            res
        };
        // println!("{} {}", first, last);
        (last - 0).max(n-1 - first) as i32
    }
}