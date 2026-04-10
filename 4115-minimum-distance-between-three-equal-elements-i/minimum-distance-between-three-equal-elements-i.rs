impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut arr: Vec<(i32, i32, i32)> = vec![(-1, -1, -1); n+1]; // 3rdlast, 2ndlast, last
        let mut ans = i32::MAX;
        for (i, num) in nums.iter().enumerate() {
            let (l3, l2, l) = &mut arr[*num as usize];
            *l3 = *l2;
            *l2 = *l;
            *l = i as i32;
            if *l3 != -1 {
                ans = ans.min(2*(*l - *l3));
            }
        }
        if ans == i32::MAX { -1 } else { ans }
    }
}