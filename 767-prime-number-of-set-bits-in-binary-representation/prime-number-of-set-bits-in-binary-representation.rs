impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let mut ans = 0;
        for num in left..=right{
            let popcount = num.count_ones();
            ans += if primes.contains(&popcount) { 1 } else { 0 };
        }
        ans
    }
}