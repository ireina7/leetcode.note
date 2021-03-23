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
use std::mem::swap;
type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn recover_tree(root: &mut Tree) {
        let (mut fst, mut snd) = (None, None);
        let mut pre = None;
        Self::search(root, &mut fst, &mut snd, &mut pre);
        swap(
            &mut fst.unwrap().borrow_mut().val,
            &mut snd.unwrap().borrow_mut().val,
        );
    }

    fn search(node: &Tree, fst: &mut Tree, snd: &mut Tree, pre: &mut Tree) {
        if let Some(real_node) = node {
            let real_node_ref = real_node.as_ref().borrow();
            Self::search(&real_node_ref.left, fst, snd, pre);
            if let Some(prev_node) = pre {
                if prev_node.as_ref().borrow().val >= real_node_ref.val {
                    if fst.is_none() {
                        *fst = Some(prev_node.clone());
                    }
                    if fst.is_some() {
                        *snd = Some(real_node.clone());
                    }
                }
            }
            *pre = Some(real_node.clone());
            Self::search(&real_node_ref.right, fst, snd, pre);
        }
    }
}
