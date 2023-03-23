pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy1 = ListNode::new(-1);
        let mut dummy2 = ListNode::new(-1);
        let mut r1 = &mut dummy1;
        let mut r2 = &mut dummy2;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            if node.val < x {
                r1.next = Some(node);
                r1 = r1.next.as_mut().unwrap();
            } else {
                r2.next = Some(node);
                r2 = r2.next.as_mut().unwrap();
            }
        }
        r1.next = dummy2.next;
        dummy1.next
    }

    // Not recommend: need deal with boundary conditions
    pub fn partition2(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy1: Option<Box<ListNode>> = None;
        let mut dummy2: Option<Box<ListNode>> = None;
        let mut r1 = &mut dummy1;
        let mut r2 = &mut dummy2;
        while head.is_some() {
            if head.as_ref().unwrap().val < x {
                if r1.is_some() {
                    r1 = &mut r1.as_mut().unwrap().next;
                }
                std::mem::swap(r1, &mut head);
                std::mem::swap(&mut r1.as_mut().unwrap().next, &mut head);
            } else {
                std::mem::swap(r2, &mut head);
                std::mem::swap(&mut r2.as_mut().unwrap().next, &mut head);
                r2 = &mut r2.as_mut().unwrap().next;
            }
        }
        if r1.is_some() {
            r1.as_mut().unwrap().next = dummy2;
            dummy1
        } else {
            dummy2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
