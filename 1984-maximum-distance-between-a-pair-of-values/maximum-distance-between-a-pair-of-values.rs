// impl Solution {
//     pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
//         let (m,n) = (nums1.len(), nums2.len());
//         let (mut i, mut j) = (0, 0);
//         while i < m && j < n {
//             if nums1[i] < nums2[j] {
//                 i +=1;
//             }
//             else if nums1[i] > nums2[j] {
//                 j += 1;
//             } else {
//                 ans = max()
//             }
//         }
//     }
// }

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut j = 0;

        for (i, &num1) in nums1.iter().enumerate() {
            j += nums2[j..].iter().take_while(|&&num2| num2 >= num1).count();
            result = result.max(j.saturating_sub(i + 1));
        }

        result as i32
    }
}