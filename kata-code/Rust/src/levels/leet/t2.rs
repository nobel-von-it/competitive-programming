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
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if let Some(l1) = l1 {
        println!("...");
        while let Some(ref bnext) = l1.next {
            println!("val is {}", bnext.val)
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::{add_two_numbers, ListNode};

    #[test]
    fn test_add_two_numbers() {
        let mut l1 = Some(Box::new(ListNode::new(5)));
        println!("...");
        if let Some(ref mut node) = l1 {
            println!("...");
            node.next = Some(Box::new(ListNode::new(10)));
        }
        add_two_numbers(l1.clone(), None);
    }
}
