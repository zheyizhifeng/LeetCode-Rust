/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */

// @lc code=start
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
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut cur = &mut head;
        let mut tuple = (l1, l2, 0, 0); // l1, l2, bit, carry
        loop {
            tuple = match tuple {
                (None, None, _, 0) => break,
                (None, None, _, carry) => (None, None, carry, 0), // carry == 1
                (Some(n), None, _, carry) | (None, Some(n), _, carry) => (n.next, None, (n.val + carry) % 10, (n.val + carry) / 10),
                (Some(n1), Some(n2), _, carry) => (n1.next, n2.next, (n1.val + n2.val + carry) % 10, (n1.val + n2.val + carry) / 10),
            };
            *cur = Some(Box::new(ListNode::new(tuple.2)));
            cur = &mut cur.as_mut().unwrap().next;
        }
        head
    }
}


// @lc code=end
