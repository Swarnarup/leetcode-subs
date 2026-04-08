// struct Robot {
//     x: i32,
//     y: i32,
//     d: usize,
//     m: i32,
//     n: i32
// }


// /** 
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl Robot {
//     const dirs: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
//     fn new(width: i32, height: i32) -> Self {
//         Robot {
//             x = 0,
//             y = 0,
//             d: 0,
//             m = width,
//             n = height
//         }
//     }
    
//     fn step(&self, num: i32) {
//         let (mut next_x, mut next_y) = (self.x, self.y);
//         let (dx, dy) = dirs[self.d];
//         while num > 0 {
//             next_x = (self.x + dx*num).min(self.m);
//             next_y = (self.y + dy*num).min(self.n);
//             num -= ()
//         }
//     }
    
//     fn get_pos(&self) -> Vec<i32> {
        
//     }
    
//     fn get_dir(&self) -> String {
        
//     }
// }

/**
 * Your Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */



 //very cleaver solution.. walk on a line and update position infering from it


 struct Robot {
    width: i32,
    height: i32,
    num: i32,
}

#[allow(unused)]
impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            num: 0,
        }
    }

    fn step(&mut self, num: i32) {
        self.num = (self.num + num + self.width * 2 + self.height * 2 - 5)
            % (self.width * 2 + self.height * 2 - 4)
            + 1;
    }

    fn get_pos(&self) -> Vec<i32> {
        if self.num < self.width {
            vec![self.num, 0]
        } else if self.num < self.width + self.height - 1 {
            vec![self.width - 1, self.num - self.width + 1]
        } else if self.num < self.width * 2 + self.height - 2 {
            vec![self.width * 2 + self.height - 3 - self.num, self.height - 1]
        } else {
            vec![0, self.width * 2 + self.height * 2 - 4 - self.num]
        }
    }

    fn get_dir(&self) -> String {
        if self.num < self.width {
            "East".to_string()
        } else if self.num < self.width + self.height - 1 {
            "North".to_string()
        } else if self.num < self.width * 2 + self.height - 2 {
            "West".to_string()
        } else {
            "South".to_string()
        }
    }
}