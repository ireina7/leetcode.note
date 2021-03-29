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
fn add(head: Option<Box<ListNode>>, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut tail = tail;

    while let Some(mut new_tail) = head.take() {
        head = new_tail.next.take();
        new_tail.next = tail.take();
        tail = Some(new_tail);
    }
    tail
}

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tail = &mut head;
        for _ in 0..k {
            match tail.as_mut() {
                None => return head,
                Some(tail_ref) => tail = &mut tail_ref.next,
            }
        }
        let tail = tail.take();
        add(head, Solution::reverse_k_group(tail, k))
    }
}
