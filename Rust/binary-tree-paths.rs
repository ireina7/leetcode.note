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
    pub fn binary_tree_paths(root: Tree) -> Vec<String> {
        fn is_leaf(node: &Tree) -> bool {
            match node {
                None => false,
                Some(n) => {
                    let n = n.borrow();
                    n.left == None && n.right == None
                }
            }
        }
        fn traverse(root: &Tree) -> Vec<String> {
            match root {
                Some(n) if is_leaf(root) => vec![format!("{}", n.borrow().val)],
                Some(n) => {
                    let n = n.borrow();
                    let l = traverse(&n.left);
                    let r = traverse(&n.right);
                    l.into_iter()
                        .chain(r.into_iter())
                        .map(|s| format!("{}{}{}",
                            n.val, (if s == "" {""} else {"->"}), s))
                        .collect()
                },
                None => vec![]
            }
        }
        traverse(&root)
    }
}
