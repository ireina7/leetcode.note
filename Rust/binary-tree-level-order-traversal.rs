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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();

        fn traverse(
            root: &Option<Rc<RefCell<TreeNode>>>,
            res: &mut Vec<Vec<i32>>,
            level: usize
        ) {
            if let Some(r) = root {
                if res.len() == level {
                    res.push(Vec::new());
                }
                res[level].push(r.borrow().val);
                traverse(&r.borrow().left , res, level + 1);
                traverse(&r.borrow().right, res, level + 1);
            }
        }
        traverse(&root, &mut res, 0);
        res
    }
}
