use std::fmt::Display;

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
    pub fn add(&mut self, val: i32) {
        self.next = make_opt_bx(val)
    }
}
impl Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.next.is_some() {
            write!(f, "LN={value} and some is", value = self.val)
        } else {
            write!(f, "LN={value} and none is", value = self.val)
        }
    }
}
pub fn is_palindrome_node_optimized(head: Option<Box<ListNode>>) -> bool {
    let mut values = Vec::new();
    let mut current = head.as_ref();

    while let Some(node) = current {
        values.push(node.val);
        current = node.next.as_ref();
    }

    let (mut left, mut right) = (0, values.len() - 1);

    while left < right {
        if values[left] != values[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}
pub fn is_palindrome_node(head: Option<Box<ListNode>>) -> bool {
    let mut head = head;
    let mut values = vec![];

    while head.clone().is_some() {
        values.push(head.clone().unwrap().val);
        head = head.clone().unwrap().next;
    }
    let (mut left, mut right) = (0, values.len() - 1);

    while left < right {
        if values[left] != values[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}
pub fn make_opt_bx(val: i32) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode::new(val)))
}
