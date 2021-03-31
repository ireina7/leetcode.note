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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&inorder, &postorder)
    }
    fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        postorder.last().map(|root| {
            let pivot_idx = inorder
                .iter()
                .enumerate()
                .find(|(_, v)| v == &root)
                .unwrap()
                .0;
            Rc::new(RefCell::new(TreeNode {
                val  : *root,
                left : Self::helper(
                    &inorder[0..pivot_idx],
                    &postorder[0..(pivot_idx)]),
                right: Self::helper(
                    &inorder[(pivot_idx + 1)..],
                    &postorder[(pivot_idx)..postorder.len()-1])
            }))
        })
    }
}
