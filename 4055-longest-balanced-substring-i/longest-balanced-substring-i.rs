// impl Solution {
//     pub fn longest_balanced(s: String) -> i32 {
//         let mut freq = vec![0; 26];
//         for i in s.as_bytes(){
//             freq[(i - 'a' as u8) as usize] += 1;
//         }
//         freq.sort();
//         let mut ans = 0;
//         for i in (0..26).rev(){
//             ans = ans.max((26 - i) * freq[i as usize]);
//         }
//         ans
//     }
// }


impl Solution {
  pub fn longest_balanced(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut res = 0;

    for i in 0..bytes.len() {
      let mut cnts = [0; 26];

      for j in i..bytes.len() {
        let pos = (bytes[j] - b'a') as usize;
        cnts[pos] += 1;

        let mut is_ok = true;
        let mut expected = 0;

        for &count in &cnts {
          if count != 0 {
            if expected == 0 {
              expected = count;
            } else if count != expected {
              is_ok = false;
              break;
            }
          }
        }

        if is_ok {
          res = res.max(j - i);
        }
      }
    }

    (res + 1) as i32
  }
}