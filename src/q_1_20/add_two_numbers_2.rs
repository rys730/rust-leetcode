use std::sync::Arc;

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

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res_ln = Box::new(ListNode::new(0));
    let mut _l1 = l1;
    let mut _l2 = l2;
    let mut curr_res = &mut res_ln;
    let mut carry_over = 0;
    while _l1.is_some() || _l2.is_some() {
        let sum = carry_over + _l1.as_ref().map_or(0, |node| node.val)
            + _l2.as_ref().map_or(0, |node| node.val);
        carry_over = sum/10;
        curr_res.next = Some(Box::new(ListNode::new(sum%10)));
        curr_res = curr_res.next.as_mut().unwrap();
        _l1 = _l1.and_then(|node| node.next);
        _l2 = _l2.and_then(|node| node.next);
    }
    if carry_over > 0 {
        curr_res.next = Some(Box::new(ListNode::new(carry_over)))
    }
    res_ln.next
}

#[test]
fn test() {
    let l1 = Some(Box::new(ListNode{val: 6, next: 
        Some(Box::new(ListNode{val:3, next: None}))}));
    let l2 = l1.clone();
    let res = add_two_numbers(l1, l2);
    dbg!(res);
}