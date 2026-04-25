// impl Solution {
//     pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
//         gggggggggggggggggg
//     }
// }


impl Solution {
    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = points.len();
        let side64 = side as i64;
        let mut pos: Vec<i64> = Vec::with_capacity(n);
        
        // Map each point to its perimeter coordinate as an i64.
        for p in points {
            let x = p[0] as i64;
            let y = p[1] as i64;
            let coordinate = if y == 0 {
                x
            } else if x == side64 {
                side64 + y
            } else if y == side64 {
                2 * side64 + (side64 - x)
            } else {  // x == 0
                3 * side64 + (side64 - y)
            };
            pos.push(coordinate);
        }
        pos.sort_unstable();
        
        let L = 4 * side64;
        // Build the extended array.
        let mut ext: Vec<i64> = Vec::with_capacity(2 * n);
        ext.extend_from_slice(&pos);
        for &p in &pos {
            ext.push(p + L);
        }
        
        let n_usize = n;  // n as usize.
        let k_usize = k as usize;
        
        // Define a closure that checks if we can place k points with gap >= d.
        let can_place = |d: i64| -> bool {
            for start in 0..n_usize {
                let limit = start + n_usize;
                let mut cur = start;
                let mut last = ext[start];
                let mut valid = true;
                // Try to place k-1 additional points.
                for _ in 1..k_usize {
                    let target = last + d;
                    // Search in the current slice using partition_point.
                    let slice = &ext[cur + 1..limit];
                    let pos_in_slice = slice.partition_point(|&x| x < target);
                    if pos_in_slice == slice.len() {
                        valid = false;
                        break;
                    }
                    cur = cur + 1 + pos_in_slice;
                    last = ext[cur];
                }
                if valid && (ext[start] + L - last) >= d {
                    return true;
                }
            }
            false
        };
        
        let mut low = 0;
        let mut high = 2 * side64;
        while low < high {
            let mid = (low + high + 1) / 2;
            if can_place(mid) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        low as i32
    }
}