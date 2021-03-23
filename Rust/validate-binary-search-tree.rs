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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        //To avoid bound limit, I use i64 here. You can also use Option<i32>.
        fn search(root: &Option<Rc<RefCell<TreeNode>>>, lower: i64, upper: i64) -> bool {
            match root {
                None => true,
                Some(x) => {
                    let x = x.borrow();
                    if !((x.val as i64) > lower && (x.val as i64) < upper) {
                        return false;
                    }
                    search(&x.left, lower, x.val as i64) &&
                    search(&x.right, x.val as i64, upper)
                }
            }
        }
        search(&root, std::i64::MIN, std::i64::MAX)
    }
}
