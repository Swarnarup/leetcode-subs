// impl Solution {
//     pub fn min_jumps(nums: Vec<i32>) -> i32 {
        
//     }
// }

use std::collections::{HashSet, HashMap, BinaryHeap};

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {return 0;}
        let mut prm: HashMap<i32, HashSet<usize>> = HashMap::with_capacity(n);
        let mut first_idxp = n - 1;
        for i in 0..n-1 {
            if Self::isprime(nums[i]) {
                first_idxp = first_idxp.min(i);
                prm.entry(nums[i]).and_modify(|x| {x.insert(i - first_idxp);}).or_insert(HashSet::from([i - first_idxp]));
            }
        }

        if prm.is_empty() {return (n - 1) as _;}
        let mut primes = vec![2];
        for num in (3..1000).step_by(2).filter(|x| Self::isprime(*x)) {primes.push(num);}

        let nums = &nums[first_idxp..];
        let n = nums.len();
        let mut d = vec![1 - n as i32; n];
        let mut heap = BinaryHeap::from([(0, nums.len()-1)]);

        while let Some((cd, i)) = heap.pop() {
            if cd < d[i] {continue;}
            let mut r = HashSet::new();
            if i == 0 {r.insert(1);}
            else if i >= nums.len() - 2 {r.insert(i - 1);}
            else {r.insert(i - 1); r.insert(i + 1);}
            for p in Self::factorize(&primes, nums[i]) {
                if let Some(x) = prm.get(&p) {
                    r = &r | x;
                    if p == nums[i] {r.remove(&i);}
                }   
            }
            if r.contains(&0) {return (cd - 1i32).abs() + first_idxp as i32;}
            for u in r {
                if cd - 1 > d[u] {d[u] = cd - 1;heap.push((cd - 1, u));}
            }
        }
        0
    }

    fn isprime(x:i32) -> bool {
        if x == 2 {return true;}
        if x == 1 || x % 2 == 0 {return false;}
        let mut k = 3;
        while k * k <= x {
            if x % k == 0 {return false;}
            k += 2;
        }
        true
    }

    fn factorize(primes: &Vec<i32>, mut n: i32) -> HashSet<i32> {
        let mut rslt = HashSet::new();
        for p in primes {
            if *p > n {break;}
            while n % *p == 0 {rslt.insert(*p);n /= *p;} 
        }
        if n > 1 {rslt.insert(n);}
        rslt
    }

}