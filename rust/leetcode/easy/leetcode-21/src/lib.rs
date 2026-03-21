pub struct Solution;

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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut curr1 = list1;
        let mut curr2 = list2;
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy_head;

        while let (Some(mut node1), Some(mut node2)) = (curr1.as_mut(), curr2.as_mut()) {
            if node1.val <= node2.val {
                let mut next = node1.next.take();
                tail.as_mut().unwrap().next = curr1.take();
                curr1 = next;
            } else {
                let next = node2.next.take();
                tail.as_mut().unwrap().next = curr2.take();
                curr2 = next;
            }
            tail = &mut tail.as_mut().unwrap().next;
        }
        tail.as_mut().unwrap().next = curr1.or(curr2);
        dummy_head.unwrap().next
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    // ===== LeetCode Examples 여기에 추가 =====
    // #[test] fn example_1() { assert_eq!(Solution::함수명(args), expected); }
}
