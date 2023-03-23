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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = None;
        let mut cur = &mut dummy;
        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                // move list1 to cur
                std::mem::swap(cur, &mut list1);
                // move cur.next to list1
                std::mem::swap(&mut cur.as_mut().unwrap().next, &mut list1);
            } else {
                std::mem::swap(cur, &mut list2);
                std::mem::swap(&mut cur.as_mut().unwrap().next, &mut list2);
            }
            // update cur
            cur = &mut cur.as_mut().unwrap().next;
        }

        if list1.is_some() {
            std::mem::swap(cur, &mut list1);
        }
        if list2.is_some() {
            std::mem::swap(cur, &mut list2);
        }

        dummy
    }

    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        let mut cur = &mut dummy;
        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                cur.next = list1.take();
                cur = cur.next.as_mut().unwrap();
                list1 = cur.next.take();
            } else {
                cur.next = list2.take();
                cur = cur.next.as_mut().unwrap();
                list2 = cur.next.take();
            }
        }

        if list1.is_some() {
            cur.next = list1.take();
        }
        if list2.is_some() {
            cur.next = list2.take();
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
