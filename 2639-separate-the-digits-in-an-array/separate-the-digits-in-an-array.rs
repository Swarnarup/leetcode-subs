impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        for i in nums {
            ans.extend(getDs(i));
        }
        ans
    }
}

fn getDs(mut i: i32) -> Vec<i32> {
    let mut arr = vec![];
    while i > 0 {
        arr.push(i%10);
        i /= 10;
    }
    arr.reverse();
    arr
}