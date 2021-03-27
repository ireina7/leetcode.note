// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

/// Naive recursion (may be the worst...)
impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let i = lists
            .iter()
            .enumerate()
            .min_by_key(|(_, x)| x.as_ref().map_or(std::i32::MAX, |x| x.val))?
            .0;
        let mut h = lists[i].take()?;
        lists[i] = h.next;
        h.next = Self::merge_k_lists(lists);
        Some(h)
    }
}
