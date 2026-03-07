// impl Solution {
//     pub fn min_flips(s: String) -> i32 {
//         let mut last = '0' as u8;
//         let s1 = s.as_bytes().iter().fold(0, |acc, x| {
//             let mut add = 0;
//             if x == &last { add = 1; }
//             last = if last == '0' as u8 { '1' as u8 } else { '0' as u8 };
//             acc + add
//         });
//         last = '1' as u8;
//         let s2 = s.as_bytes().iter().fold(0, |acc, x| {
//             let mut add = 0;
//             if x == &last { add = 1; }
//             last = if last == '0' as u8 { '1' as u8 } else { '0' as u8 };
//             acc + add
//         });

//         s1.min(s2)
//     }
// }


use std::cmp::min;
impl Solution {
    pub fn min_flips(mut s: String) -> i32 {
        let len = s.len();
        let mut chars = s.chars().collect::<Vec<char>>();
        let ch = ['0','1'];
        let mut cnt = 0;
        for i in 0..len {
            if chars[i] != ch[i%2] {
                cnt+=1;
            }
        }
        let mut ans = min(cnt,len as i32 - cnt);
        for i in 0..len {
            if chars[i] != ch[i%2] {
                cnt-=1;
            }
            if chars[i] != ch[(i+len)%2] {
                cnt+=1;
            }
            ans = ans.min(min(cnt,len as i32 - cnt));
        }
        return ans;
    }
}


