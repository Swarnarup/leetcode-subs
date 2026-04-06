// use std::collections::HashMap;
// use std::collections::HashSet;

// impl Solution {
//     pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
//         let mut pos = (0, 0);
//         let mut dir = (0, 1);
//         let mut next_pos = (0, 0);

//         let mut along_this_x: HashMap<i32, HashSet<i32>> = HashMap::new();
//         let mut along_this_y: HashMap<i32, HashSet<i32>> = HashMap::new();

//         let mut ans = 0;

//         let mut begun = false;

//         for arr in obstacles {
//             let (x, y) = (arr[0], arr[1]);
//             along_this_x.entry(x).or_insert(HashSet::new()).insert(y);
//             along_this_y.entry(y).or_insert(HashSet::new()).insert(x);
//         }
// println!("{:?}", &along_this_x);
// println!("{:?}", &along_this_y);
//         for cmd in commands {
//             if cmd < 0 {
//                 update_dir(&mut dir, cmd);
//                 continue;
//             }
//             if dir.0 != 0 {
//                 for i in range(pos.0, pos.0 + dir.0*cmd) {
//                     match along_this_y.get(&pos.1) {
//                         Some(st) => {
//                             if st.contains(&i) && !begun {
//                                 break;
//                             }
//                         },
//                         _ => ()
//                     }
//                     begun = true;
//                     next_pos.0 = i;
//                 }
//             }
//             else if dir.1 != 0 {
//                 for i in range(pos.1, pos.1 + dir.1*cmd) {
//                     match along_this_x.get(&pos.0) {
//                         Some(st) => {
//                             if st.contains(&i) && !begun {
//                                 break;
//                             }
//                         },
//                         None => ()
//                     };
//                     begun = true;
//                     next_pos.1 = i;
//                 }
//             }

//             pos = next_pos;
//             println!("DIr: {:?}", &dir);
//             println!("{:?}", &pos);
//             ans = ans.max((pos.0*pos.0) + (pos.1*pos.1));
//             begun = true;
//         }

//         ans
//     }
// }

// fn update_dir(dir: &mut (i32, i32), turn: i32) {
//     match turn {
//         -1 => rotate_rt(dir),
//         _ => rotate_lt(dir),
//     }
// }

// fn rotate_lt(dir: &mut (i32, i32)) {
//     *dir = (-(dir.1), (dir.0));
// }
// fn rotate_rt(dir: &mut (i32, i32)) {
//     *dir = ((dir.1), -(dir.0));
// }

// fn range(a: i32, b: i32) -> impl Iterator<Item = i32> {
//     (a.min(b))..=(b.max(a))
// }


use std::collections::HashSet;


impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let oset: HashSet<(i32, i32)> = obstacles.iter().map(|x| (x[0], x[1])).collect();
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut d: usize = 0;
        let ds: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut ans: i32 = 0;
        for cmd in commands {
            match cmd {
                -2 => {
                    d = (d + 3) % 4;
                }
                -1 => {
                    d = (d + 1) % 4;
                }
                _ => {
                    for _mv in 0..cmd {
                        let candidate = (x + ds[d].0, y + ds[d].1);
                        if oset.contains(&candidate) {
                            break;
                        }
                        (x, y) = candidate;
                    }
                    ans = ans.max(x * x + y * y);
                }
            }
        }
        ans
    }
}