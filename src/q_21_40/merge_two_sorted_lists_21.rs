// Definition for singly-linked list.
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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res_ln = Box::new(ListNode::new(0));
        let mut l1 = list1;
        let mut l2 = list2;
        let mut curr_res_node = &mut res_ln;
        while l1.is_some() || l2.is_some(){
            let l1_val = l1.as_ref().map_or(101, |node| node.val);
            let l2_val = l2.as_ref().map_or(101, |node| node.val); 
            if l1_val < l2_val {
                curr_res_node.next = Some(Box::new(ListNode::new(l1_val)));
                l1 = l1.and_then(|node| node.next)
            } else {
                curr_res_node.next = Some(Box::new(ListNode::new(l2_val)));
                l2 = l2.and_then(|node| node.next)
            }
            curr_res_node = curr_res_node.next.as_mut().unwrap();

        }
        res_ln.next
    }
}
struct Solution{}
