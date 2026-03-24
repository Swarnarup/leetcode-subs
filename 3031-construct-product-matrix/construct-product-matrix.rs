// const md: i32 = 12345;

// impl Solution {
//     pub fn construct_product_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         let mut mul = 1;
//         for arr in &grid {
//             for i in arr {
//                 mul = (mul * (i % md)) % md;
//             }
//         }
//         println!("{}", mul);
//         let inv = precompute_inverses(md);
//         for arr in grid.iter_mut() {
//             for i in arr {
//                 let x = *i;
//                 *i = mod_divide_general(mul, x, &inv);
//             }
//         }

//         grid
//     }
// }

// fn precompute_inverses(m: i32) -> Vec<i32> {
//     let mut inv = vec![0; (m + 1) as usize];
//     inv[1] = 1;
//     for i in 2..=m as usize {
//         // Formula: inv[i] = -(m/i) * inv[m%i] % m
//         inv[i] = (m - (m / i as i32) * inv[m as usize % i] % m) % m;
//     }
//     inv
// }

// fn mod_divide_general(a: i32, b: i32, inv: &Vec<i32>) -> i32 {
//     let a_pos = a % md;
//     let b_pos = if b % md == 0 {b} else {b % md};
//     (a_pos * inv[b_pos as usize]) % md
// }



impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut p = 1;
        let (m, n) = (grid.len(), grid[0].len());
        let mut product = vec![vec![1_u64; n]; m];
        
        for i in 0..m{
            for j in 0..n{
                product[i][j] *= p;
                product[i][j] %= 12345;
                p *= grid[i][j] as u64;
                p%=12345;
            }
        }
        
        p = 1;
        
        for i in (0..m).rev(){
            for j in (0..n).rev(){
                product[i][j] *= p;
                product[i][j] %= 12345;
                p *= grid[i][j] as u64;
                p%=12345;
            }
        }

        return product.into_iter()
            .map(|row| row.into_iter()
                    .map(|cell| cell as i32)
                    .collect::<Vec<i32>>()
            ).collect::<Vec<Vec<i32>>>()
    }
}