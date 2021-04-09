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

/// Naive String solution
use std::rc::Rc;
use std::cell::RefCell;
type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn sum_numbers(root: Tree) -> i32 {
        fn traverse(root: &Tree) -> Vec<String> {
            match root {
                None => vec![],
                Some(node) => {
                    let node = node.borrow();
                    if node.left.is_none() && node.right.is_none() {
                        return vec![node.val.to_string()];
                    }
                    traverse(&node.left)
                        .into_iter()
                        .chain(traverse(&node.right))
                        .map(|s| format!("{}{}", node.val, s))
                        .collect()
                },
            }
        }
        traverse(&root)
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .sum()
    }
}

/// Pure number solution
use std::rc::Rc;
use std::cell::RefCell;
type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn sum_numbers(root: Tree) -> i32 {
        fn traverse(root: &Tree, sum: i32) -> i32 {
            root.as_ref().map(|node| {
                let node = node.borrow();
                let next = sum * 10 + node.val;
                if node.left.is_none() & node.right.is_none() {
                    return next;
                }
                traverse(&node.left, next) + traverse(&node.right, next)
            }).unwrap_or(0)
        }
        traverse(&root, 0)
    }
}
