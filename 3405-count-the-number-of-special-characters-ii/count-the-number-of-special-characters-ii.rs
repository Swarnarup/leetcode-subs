// Unlike Count number of special characters I, this problem is best
// solved using states..
use State::*;
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut present: Vec<State> = vec![Unchecked; 26];  
        let mut ans = 0;
        // println!("{}", &word);
        for ch in word.chars() {
            // println!("{} ------ {:?}", ch, &present);
            if ch as usize >= 'a' as usize {
                if present[ch as usize - 'a' as usize] == Stale {
                    continue;
                }
                if present[ch as usize - 'a' as usize] == LowToUp {
                    ans -= 1;
                    present[ch as usize - 'a' as usize] = Stale;
                }
                else {
                    present[ch as usize - 'a' as usize] = GotLow;
                }
            }
            else {
                if present[ch as usize - 'A' as usize] == Stale || present[ch as usize - 'A' as usize] == LowToUp {
                    continue;
                }
                if present[ch as usize - 'A' as usize] == GotLow {
                    ans += 1;
                    present[ch as usize - 'A' as usize] = LowToUp;
                }
                else { present[ch as usize - 'A' as usize] = Stale; }
            }
        }

        ans
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    Unchecked,
    Stale,  //the lowercase after uppercase
    GotLow,
    LowToUp
}