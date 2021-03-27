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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn traverse(
            root: &Option<Rc<RefCell<TreeNode>>>,
            memo: &mut Vec<Vec<i32>>,
            level: usize,
        ) {
            if let Some(r) = root {
                if memo.len() == level {
                    memo.push(vec![]);
                }
                memo[level].push(r.borrow().val);
                traverse(&r.borrow().left , memo, level + 1);
                traverse(&r.borrow().right, memo, level + 1);
            }
        }
        let mut memo: Vec<Vec<i32>> = Vec::new();
        traverse(&root, &mut memo, 0);
        let mut ord = 0;
        memo.into_iter().map(|v| { match ord {
            0 => { ord = 1; v },
            _ => { ord = 0; v.into_iter().rev().collect() }
        }}).collect()
    }
}
