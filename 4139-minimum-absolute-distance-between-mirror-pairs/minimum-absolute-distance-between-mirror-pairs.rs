use std::collections::HashMap;
impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut index: HashMap<i32, usize> = HashMap::new();
        let mut ans = i32::MAX;
        for (idx, i) in nums.iter().rev().enumerate() {
            let r = rev(*i);
                // println!("{} {}", idx, r);
            if let Some(idx2) = index.get(&r) {
                ans = ans.min((idx - *idx2) as i32);
            }
            *index.entry(*i).or_insert(idx) = idx;
        }
        // println!("{:?}", index);
        if ans != i32::MAX { ans } else { -1 }
    }
}

fn rev(mut x: i32) -> i32 {
    let mut n = 0;
    while x > 0 {
        n *= 10;
        n += x % 10;
        x = x / 10;
    }
    n
}

fn remove_leading0(mut x: i32) -> i32 {
    while x > 0 && x % 10 == 0 {
        x = x / 10;
    }
    x
}