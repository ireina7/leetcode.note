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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn search(root: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> bool {
            match root {
                None => false,
                Some(x) => {
                    let x = x.borrow();
                    let t = target - x.val;
                    if let (None, None) = (&x.left, &x.right) {
                        return t == 0;
                    }
                    search(&x.left, t) || search(&x.right, t)
                }
            }
        }
        search(&root, target_sum)
    }
}
