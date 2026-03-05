impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut last = '0' as u8;
        let s1 = s.as_bytes().iter().fold(0, |acc, x| {
            let mut add = 0;
            if x == &last { add = 1; }
            last = if last == '0' as u8 { '1' as u8 } else { '0' as u8 };
            acc + add
        });
        last = '1' as u8;
        let s2 = s.as_bytes().iter().fold(0, |acc, x| {
            let mut add = 0;
            if x == &last { add = 1; }
            last = if last == '0' as u8 { '1' as u8 } else { '0' as u8 };
            acc + add
        });

        s1.min(s2)
    }
}