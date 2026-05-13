// impl Solution {
//     pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        
//     }
// }
impl Solution {
    pub fn min_moves(n: Vec<i32>, l: i32) -> i32 {
        let (mut d, l) = ([0; 200002], l as usize);
        for i in 0..n.len()/2 {
            let (a, b) = (n[i] as usize, n[n.len()-1-i] as usize);
            d[a.min(b)+1]-=1;d[a.max(b)+l+1]+=1; d[a+b]-=1;d[a+b+1]+=1
        }
        (2..2*l+1).fold((0,0), |(min, c),i|(min.min(c+d[i]),c+d[i])).0 +n.len() as i32
    }
}