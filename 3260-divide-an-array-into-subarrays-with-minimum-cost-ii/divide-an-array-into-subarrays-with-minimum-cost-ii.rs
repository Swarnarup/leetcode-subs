// impl Solution {
//     pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        
//     }
// }

impl Solution {
	pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
		let mut lower = std::collections::BTreeSet::<(i32, i32)>::new();
		let mut upper = lower.clone();
		let mut s = nums[0] as i64;
		let mut ans = i64::MAX;
		for (j, &n) in (1..).zip(&nums[1..]) {
			let i = j - dist - 1;
			if i >= 1 {
				let prv = nums[i as usize];
				if !upper.remove(&(prv, i)) {
					s -= prv as i64;
					lower.remove(&(prv, i));
					let mm = upper.pop_first().unwrap();
					s += mm.0 as i64;
					lower.insert(mm);
				}
			}
			lower.insert((n, j));
			s += n as i64;
			if lower.len() as i32 + 1 == k {
				ans = ans.min(s);
				let mm = lower.pop_last().unwrap();
				s -= mm.0 as i64;
				upper.insert(mm);
			}
		}
		ans
	}
}