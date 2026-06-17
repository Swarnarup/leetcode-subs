impl Solution {
    pub fn process_str(s: String, mut k: i64) -> char {
        // Recursion | catch: need to calculate the total length beforehand..
        // iterate from back and reduce len until you reach k == len
        let mut len = 0;
        for ch in s.chars() {
            match ch {
                '*' => len = 0.max(len - 1),
                '#' => len = 2 * len,
                '%' => (),
                _ => len += 1
            };
        }
        println!("{}", len);
        let mut last = '.';
        for ch in s.chars().rev() {
            if len <= k {
                break;
            }
            match ch {
                '*' => {
                    len += 1;
                },
                '#' => {
                    len = len/2;
                    if k >= len {
                        k = k - len;
                    }
                },
                '%' => {
                    k = len - k - 1;
                },
                _ => {
                    last = ch;
                    len -= 1;
                }
            };
            // println!("{} {}", k, len);
        }
        last
    }
}
// 0 1 2 3 4 5
// c d % # * # 6,3  3,3   

// cd dc dcdc dcd dcddcd 