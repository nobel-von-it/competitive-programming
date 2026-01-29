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
fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(ref h) = head {
        if h.next.is_none() {
            return head;
        }
    }
    let mut h1 = head.clone().as_mut().unwrap();
    let mut h2 = head.as_ref().unwrap().next.as_ref().unwrap();

    // while h2.next.is_some() {
    //     let new_node = ListNode::new(gcd(h1.val, h2.val));
    //     h1.next = Some(Box::new(new_node));
    // }
    None
}
