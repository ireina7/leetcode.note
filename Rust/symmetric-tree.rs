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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn search(
            n1: &Option<Rc<RefCell<TreeNode>>>,
            n2: &Option<Rc<RefCell<TreeNode>>>)
        -> bool {
            match (n1, n2) {
                (None, None) => true,
                (Some(x1), Some(x2)) if x1.borrow().val == x2.borrow().val => {
                    let x1 = x1.borrow();
                    let x2 = x2.borrow();
                    search(&x1.left, &x2.right) && search(&x1.right, &x2.left)
                },
                _ => false
            }
        }
        match root {
            Some(x) => {
                let x = x.borrow();
                search(&x.left, &x.right)
            },
            _ => true
        }
    }
}
