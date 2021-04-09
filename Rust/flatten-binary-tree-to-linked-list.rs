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
    pub fn flatten(root: &mut Tree) {
        if root.is_none() {
            return;
        }
        fn traverse(root: &Tree, memo: &mut Vec<Rc<RefCell<TreeNode>>>) {
            root.as_ref().map(|node| {
                memo.push(node.clone());
                let node = node.borrow();
                traverse(&node.left, memo);
                traverse(&node.right, memo);
            });
        }
        let mut memo = vec![];
        traverse(root, &mut memo);

        let mut curr = memo[0].clone();
        for i in 1..memo.len() {
            curr.borrow_mut().right = Some(memo[i].clone());
            curr.borrow_mut().left = None;
            curr = memo[i].clone();
        }
    }
}
