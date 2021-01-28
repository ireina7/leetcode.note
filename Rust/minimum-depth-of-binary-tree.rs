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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(p) => {
                let p = p.borrow();
                if p.left.is_some() && p.right.is_some() {
                    Self::min_depth(p.left.clone()).min(Self::min_depth(p.right.clone())) + 1
                }
                else {
                    Self::min_depth(p.left.clone()).max(Self::min_depth(p.right.clone())) + 1
                }
            }
        }
    }
}
