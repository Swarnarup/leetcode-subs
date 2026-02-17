impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut times = vec![];
        let turned_on = turned_on as u32;

        for h in 0..12i32 {
            for m in 0..60i32 {
                if h.count_ones() + m.count_ones() == turned_on {
                    times.push(format!("{h}:{m:0>2}"));
                }
            }
        }

        times
    }
}