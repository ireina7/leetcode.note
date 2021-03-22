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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn search(root: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Vec<i32>> {
            match root {
                None => vec![],
                Some(x) => {
                    let x = x.borrow();
                    let t = target - x.val;
                    if let (None, None) = (&x.left, &x.right) {
                        return if t == 0 { vec![vec![x.val]] } else { vec![] };
                    }
                    search(&x.left, t)
                        .into_iter()
                        .chain(search(&x.right, t))
                        .map(|xs| { vec![x.val].into_iter().chain(xs).collect() })
                        .collect()
                }
            }
        }
        search(&root, target_sum)
    }
}
