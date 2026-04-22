// impl Solution {
//     pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
//         let mut ans = Vec::new();
//         for s1 in &queries {
//             for s2 in &dictionary {
//                 println!("{}", same(s1, s2));
//                 if same(s1, s2) >= 2 {
//                     ans.push(s1.to_string());
//                     break;
//                 }
//             }
//         }
//         ans
//     }
// }

// fn same(a: &str, b: &str) -> i32 {
//     let mut f1 = vec![0i32; 26];
//     let mut f2 = vec![0i32; 26];
//     for ch in a.chars() {
//         f1[ch as usize - 'a' as usize] += 1;
//     }
//     for ch in b.chars() {
//         f2[ch as usize - 'a' as usize] += 1;
//     }
//     let mut same = 0;
//     for i in 0..26 {
//         same += f1[i].min(f2[i]);
//     }
//     same
// }


impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut res = Vec::new();
        for q in queries.iter() {
            let mut ok = false;
            for d in dictionary.iter() {
                let diff = q.chars().zip(d.chars())
                    .filter(|(a, b)| a != b)
                    .count();
                if diff <= 2 {
                    ok = true;
                    break;
                }
            }
            if ok {
                res.push(q.clone());
            }
        }
        res
    }
}