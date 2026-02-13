// impl Solution {
//     pub fn longest_balanced(s: String) -> i32 {
//         let n = s.as_bytes().len();
//         let (mut af, mut bf, mut cf) = (vec![0; n], vec![0; n], vec![0; n]);
//         let (mut a, mut b, mut c) = (0, 0, 0);
//         for ch in s.chars(){
//             if ch == 'a' { a+=1; af[a] += }
//         }
//         0
//     }
// }

use std::collections::HashMap;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let mut ans = 0;
        let mut hash = HashMap::<[i32; 3], usize>::new();
        for k in 0..7 {
            hash.insert([0; 3], 0);
            let mut arr = [0; 3];
            for (i, b) in s.bytes().enumerate() {
                arr[(b - b'a') as usize] += 1;
                let mut lo = i32::MAX;
                for d in 0..3 {
                    if (k & (1 << d)) == 0 {
                        lo = lo.min(arr[d]);
                    }
                }
                for d in 0..3 {
                    if (k & (1 << d)) == 0 {
                        arr[d] -= lo;
                    }
                }
                if let Some(&j) = hash.get(&arr) {
                    ans = ans.max(i + 1 - j);
                }
                else {
                    hash.insert(arr.clone(), i + 1);
                }
            }
            hash.clear();
        }
        ans as i32
    }
}