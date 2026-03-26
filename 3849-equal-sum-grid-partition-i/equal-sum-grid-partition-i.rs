impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut nums = vec![(0, 0, 0); (m*n)];
        for i in 0..m {
            for j in 0..n {
                nums[(i+1)*j] = (grid[i][j] as i64, i, j);
            }
        }
        let mut sm = grid.iter().fold(0 as i64, |acc, arr| {
            acc + arr.iter().fold(0 as i64, |acc, x| acc + *x as i64)
        });

        let mut curr = 0 as i64;
        let mut ii = 0;
        for arr in &grid {
            curr += arr.iter().fold(0 as i64, |acc, x| acc + *x as i64);
            let next = sm - curr;
            if curr == next { return true; }
            else {
                let diff = curr - next;
                if diff < 0 {
                    if let Some((num, i, j)) = bs_left(&nums, diff) {
                        if num == diff && i <= ii {
                            return true;
                        }
                    }
                }
            }
            ii+=1;
        }

        curr = 0 as i64;
        let mut jj = 0;
        for j in 0..n {
            let mut tmp = 0;
            for i in 0..m { tmp += grid[i][j] as i64; }
            curr += tmp;
            let next = sm - curr;
            if curr == next { return true; }
            else {
                let diff = curr - next;
                if diff < 0 {
                    if let Some((num, i, j)) = bs_left(&nums, diff) {
                        if num == diff && j <= jj {
                            return true;
                        }
                    }
                }
            }
            jj += 1;
        }
        false
    }
}

fn bs_left(arr: &Vec<(i64,usize, usize)>, target: i64) -> Option<(i64, usize, usize)> {
    let mut ans = None;
    let (mut i, mut j, mut mid) = (0, arr.len(), 0);
    while i as i32 <= j as i32 {
        mid = (i+j)/2;
        if arr[mid].0 >= target {
            ans = Some(arr[mid]);
            j = mid - 1;
        } else {
            i = mid + 1;
        }
    }
    ans
} 
fn bs_right(arr: &Vec<(i64,usize, usize)>, target: i64) -> Option<(i64,usize, usize)> {
    let mut ans = None;
    let (mut i, mut j, mut mid) = (0, arr.len(), 0);
    while i as i32 <= j as i32 {
        mid = (i+j)/2;
        if arr[mid].0 <= target {
            ans = Some(arr[mid]);
            i = mid + 1;
        } else {
            j = mid - 1;
        }
    }
    ans
} 