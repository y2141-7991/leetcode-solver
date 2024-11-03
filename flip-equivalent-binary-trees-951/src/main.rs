use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution;

impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let (node1, node2) = match (root1, root2) {
            (Some(node1), Some(node2)) => (node1, node2),
            (None, None) => return true,
            (None, _) => return false,
            (_, None) => return false,
        };
        if node1.borrow().val != node2.borrow().val {
            return false;
        }
        return Self::flip_equiv(node1.borrow().left.clone(), node2.borrow().left.clone())
            && Self::flip_equiv(node1.borrow().right.clone(), node2.borrow().right.clone())
            || Self::flip_equiv(node1.borrow().left.clone(), node2.borrow().right.clone())
                && Self::flip_equiv(node1.borrow().right.clone(), node2.borrow().left.clone());
    }
}

fn main() {
    println!("Hello, world!");
}
