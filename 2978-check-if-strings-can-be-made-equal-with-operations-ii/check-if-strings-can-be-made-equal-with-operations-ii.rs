// impl Solution {
//     pub fn check_strings(s1: String, s2: String) -> bool {
//         let mut v10: Vec<char> = Vec::new();
//         let mut v11: Vec<char> = Vec::new();
//         for (i, ch) in s1.chars().enumerate(){
//             if i&1 == 0 { v10.push(ch); }
//             else { v11.push(ch); }
//         }
//         v10.sort();
//         v11.sort();
//         let mut v20: Vec<char> = Vec::new();
//         let mut v21: Vec<char> = Vec::new();
//         for (i, ch) in s2.chars().enumerate(){
//             if i&1 == 0 { v20.push(ch); }
//             else { v21.push(ch); }
//         }
//         v20.sort();
//         v21.sort();
//         v10 == v20 && v11 == v21
//     }
// }

// frequency match
impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let mut f10: Vec<i32> = vec![0; 26];
        let mut f11: Vec<i32> = vec![0; 26];
        for (i, ch) in s1.chars().enumerate(){
            if i&1 == 0 { f10[get_idx(ch)] += 1 }
            else { f11[get_idx(ch)] += 1 }
        }
        let mut f20: Vec<i32> = vec![0; 26];
        let mut f21: Vec<i32> = vec![0; 26];
        for (i, ch) in s2.chars().enumerate(){
            if i&1 == 0 { f20[get_idx(ch)] += 1 }
            else { f21[get_idx(ch)] += 1 }
        }
        f10 == f20 && f11 == f21
    }
}

fn get_idx(i: char) -> usize {
    i as usize - 'a' as usize
}