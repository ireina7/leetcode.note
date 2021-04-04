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
    pub fn level_order_bottom(root: Tree) -> Vec<Vec<i32>> {
        fn search(root: &Tree, level: usize, memo: &mut Vec<Vec<i32>>) {
            root.as_ref().map(|node| {
                if let None = memo.get(level) {
                    memo.push(vec![]);
                }
                let node = node.borrow();
                memo[level].push(node.val);
                search(&node.left , level + 1, memo);
                search(&node.right, level + 1, memo);
            });
        }
        let mut memo: Vec<Vec<i32>> = vec![];
        search(&root, 0, &mut memo);
        memo.into_iter().rev().collect()
    }
}
