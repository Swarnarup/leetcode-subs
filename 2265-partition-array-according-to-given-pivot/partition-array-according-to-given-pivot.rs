// impl Solution {
//     pub fn pivot_array(mut nums: Vec<i32>, pivot: i32) -> Vec<i32> {
//         // 0-i -> small; i+1 - j -> equal; j+1 - k -> large
//         // analyzing the jth element

//         let (mut i, mut j, mut k) = (0, 0, nums.len()-1);
//         while j <= k {
//             if nums[j] > pivot {
//                 nums.swap(j, k);
//                 k -= 1;
//             }
//             else if nums[j] == pivot {
//                 j += 1;
//             }
//             else {
//                 nums.swap(j, i);
//                 j += 1;
//                 i += 1;
//             }
//             // println!("{:?}", &nums);
//         }
        
//         nums
//     }
// }


impl Solution {
    pub fn pivot_array(mut nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut stable_nums = Vec::with_capacity(nums.len());

        // Pass 1: Collect small elements
        for &num in &nums { if num < pivot { stable_nums.push(num); } }
        // Pass 2: Collect equal elements
        for &num in &nums { if num == pivot { stable_nums.push(num); } }
        // Pass 3: Collect large elements
        for &num in &nums { if num > pivot { stable_nums.push(num); } }
        
        stable_nums
    }
}