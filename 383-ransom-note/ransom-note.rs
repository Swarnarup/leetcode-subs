impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut f_rn = vec![0; 26];
        let mut f_mz = vec![0; 26];

        for i in ransom_note.chars() {
            f_rn[(i as usize - 'a' as usize)] += 1;
        }
        for i in magazine.chars() {
            f_mz[(i as usize - 'a' as usize)] += 1;
        }

        for i in 0..26 {
            if f_rn[i] > f_mz[i] {
                return false;
            }
        }
        true
    }
}