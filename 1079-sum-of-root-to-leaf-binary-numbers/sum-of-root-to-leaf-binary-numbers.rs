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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut t = 0;
        sum_tree(root, 0, &mut t);
        t
    }
}

fn is_leaf(nd: Rc<RefCell<TreeNode>>)->bool{
    if nd.borrow().left.is_none() && nd.borrow().right.is_none() {
        return true;
    }
    false
}

fn sum_tree(root: Option<Rc<RefCell<TreeNode>>>, mut cur: i32, total: &mut i32){
    if root.is_none() {
        return;
    }
    cur = cur << 1;
    let nd = root.unwrap();
    if nd.borrow().val == 1 {
        cur = cur | 1;
    }
    if is_leaf(nd.clone()) {
        *total += cur;
        return;
    }
    sum_tree(nd.borrow().left.clone(), cur, total);
    sum_tree(nd.borrow().right.clone(), cur, total);
}