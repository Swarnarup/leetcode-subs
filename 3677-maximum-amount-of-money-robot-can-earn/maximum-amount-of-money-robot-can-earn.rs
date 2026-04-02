// impl Solution {
//     pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        
//     }
// }

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let mut line = vec![(i32::MIN,i32::MIN,i32::MIN); coins[0].len()];
        let mut left = (0,0,0);
        line[0] = left;
        
        for crow in coins {
            left = (i32::MIN,i32::MIN,i32::MIN);   
            for (up, &coins) in line.iter_mut().zip(crow.iter()) {
                left.2 = (coins + left.2.max(up.2)).max(left.1).max(up.1);
                left.1 = (coins + left.1.max(up.1)).max(left.0).max(up.0);
                left.0 = coins + left.0.max(up.0);
                *up = left;
            }
        }
        return left.2;;
    }
}