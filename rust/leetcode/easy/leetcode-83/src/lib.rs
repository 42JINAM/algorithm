pub struct Solution;
Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // let mut current = &mut head;
        //
        // while let Some(node) = current {
        //     while node.next.is_some() && node.next.as_ref().unwrap().val == node.val {
        //         if let Some(mut next_node) = node.next.take() {
        //             node.next = next_node.next;
        //         }
        //     }
        //     current = &mut node.next;
        // }
        // return head;


        let mut curr_opt = head.as_mut();

        while let Some(curr) = curr_opt {
            let mut next_opt = curr.next.take();
            while let Some(next) = next_opt.as_mut() {
                if next.val == curr.val {
                    next_opt = next.next.take();
                } else {
                    curr.next = next_opt;
                    break;
                }
            }
            curr_opt = curr.next.as_mut();
        }
        head
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    // ===== LeetCode Examples 여기에 추가 =====
    // #[test] fn example_1() { assert_eq!(Solution::함수명(args), expected); }
}
