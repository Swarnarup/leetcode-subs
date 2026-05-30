use std::collections::BTreeSet;

impl Solution {
    // pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        
    // }
        pub fn get_results(q: Vec<Vec<i32>>) -> Vec<bool> {
        let (mut o, mut t, z) = (BTreeSet::from([0, 50005]), vec![0; 100010], 50005); t[z]=z as i32;
        q.into_iter().filter_map(|c| { let x=c[1];
            if c[0]>1 { let f=*o.range(..=x).next_back()?; let (mut l, mut r, mut m)=(z, f as usize+z-1, 0);
                while l<=r { if l%2>0 {m=m.max(t[l]); l+=1} if r%2<1 {m=m.max(t[r]); r-=1} l/=2; r/=2 }
                Some((x-f).max(m) >= c[2])
            } else { let (l, r) = (*o.range(..x).next_back()?, *o.range(x..).next()?); o.insert(x);
                for (k,v) in [(l, x-l), (x, r-x)] { let mut i=k as usize+z; t[i]=v; while i>1 {i/=2; t[i]=t[i*2].max(t[i*2+1])} }
                None
            }
        }).collect()
    }
}