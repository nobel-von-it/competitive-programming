#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, inorder_traversal: &mut Vec<i32>) {
    if let Some(node) = root {
        let n = node.borrow();
        inorder(&n.left, inorder_traversal);
        inorder_traversal.push(node.borrow().val);
        inorder(&n.right, inorder_traversal);
    }
}
fn replaceValues(root: &Option<Rc<RefCell<TreeNode>>>, inorder_traversal: &mut Vec<i32>) {
    if let Some(node) = root {
        let mut node = node.borrow_mut();

        let mut node_sum = 0;
        for i in inorder_traversal.iter() {
            if i > &node.val {
                node_sum += i;
            } else {
                break;
            }
        }

        node.val += node_sum;
        replaceValues(&node.left, inorder_traversal);
        replaceValues(&node.right, inorder_traversal);
    }
}

pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut inorder_traversal = vec![];
    inorder(&root, &mut inorder_traversal);
    inorder_traversal.reverse();
    replaceValues(&root, &mut inorder_traversal);
    root
}
