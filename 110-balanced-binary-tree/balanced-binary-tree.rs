// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (_, verdict) = depth(root);
        verdict
    }
}

fn depth(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
    // if root.is_none() { return (0, true); }
    // let (d_l, b_l) = depth(root.unwrap().borrow().left.clone());
    // if !b_l { return (0, false); }
    // let (d_r, b_r) = depth(root.unwrap().borrow().right.clone());
    // if !b_r || ((d_l - d_r).abs() > 1) { return (0, false); }
    // (1 + d_l.max(d_r), true)

    if let Some(node) = root.as_ref() {
        let (d_l, b_l) = depth(node.borrow().left.clone());
        if !b_l { return (0, false); }

        let (d_r, b_r) = depth(node.borrow().right.clone());
        if !b_r || ((d_l - d_r).abs() > 1) { return (0, false); }
        return (1 + d_l.max(d_r), true);
    }
    
    (0, true)
}