// impl Solution {
//     pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        
//     }
// }


impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
      let n = nums.len();
      #[derive(Debug,PartialEq,Clone,Copy)]
      enum Slope {
        Decreasing,
        Zero,
        Increasing
      };
      #[derive(Debug,Clone,Copy)]
      struct Interval {
        start: usize,
        end: usize,
        slope: Slope
      };
      
      fn get_slope(a:i32, b:i32) -> Slope {
        match b - a {
          dec if dec < 0 => Slope::Decreasing,
          0 => Slope::Zero,
          _ => Slope::Increasing
        }
      };

      let mut prefix:Vec<i64> = Vec::with_capacity(n + 1);
      prefix.push(0);
      prefix.push(nums[0] as i64);
      prefix.push((nums[0] + nums[1]) as i64);

      let mut intervals:Vec<Interval> = Vec::new();
      let mut part = Interval {
        start: 0, 
        end: 1, 
        slope: get_slope(nums[0], nums[1])
      };
      for i in 2..nums.len() {
        prefix.push(prefix.last().unwrap() + nums[i] as i64);
        let m_right = get_slope(nums[i-1], nums[i]);
        if m_right != part.slope {
          part.end = i-1;
          intervals.push(part);
          part = Interval { start: i-1, end: i, slope: m_right};
        }
      } 
      part.end = nums.len()-1;
      intervals.push(part);

      let mut res = i64::MIN;
      for window in intervals.windows(3) {
        let (a, b, c) = (window[0], window[1], window[2]);
        if a.slope != Slope::Increasing || 
           b.slope != Slope::Decreasing || 
           c.slope != Slope::Increasing {
            continue;
        }

        // Find the largest maximal partial sum in
        // [i..a.end) [b.start..b.end] (c.start..j]
        let (mut sum, mut begin) = (i64::MIN, a.start);
        for i in a.start..a.end {
          let window_sum = prefix[a.end+1] - prefix[i];

          if window_sum > sum {
            sum = window_sum;
            begin = i;
          }
        }

        let (mut sum, mut end) = (i64::MIN, c.start);
        for i in (c.start+1)..=c.end {
          let window_sum = prefix[i+1] - prefix[c.start];

          if window_sum > sum {
            sum = window_sum;
            end = i;
          }
        }

        res = res.max(prefix[end+1] - prefix[begin]);
      }

      res
    }
}