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

type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn is_balanced(root: Tree) -> bool {
        fn dfs(root: &Tree) -> i32 {
            match root {
                Some(root) => {
                    let root = root.borrow();
                    let left = dfs(&root.left);
                    let right = dfs(&root.right);
                    if (left - right).abs() > 1 || left == -1 || right == -1 {
                        return -1;
                    }
                    left.max(right) + 1
                }
                None => 0
            }
        }
        dfs(&root) != -1
    }
}
