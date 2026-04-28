impl Solution {
    // pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        
    // }
        pub fn min_operations(g: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut a=g.concat();a.sort();let m=a[a.len()/2];
        if a.iter().any(|v|(v-a[0])%x!=0){-1}else{a.iter().map(|v|(v-m).abs()/x).sum()}
    }
}