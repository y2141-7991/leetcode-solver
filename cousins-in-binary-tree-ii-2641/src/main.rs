use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{VecDeque, BinaryHeap};

#[derive(Debug, Clone,)]
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
    pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = match root {
            Some(node) => node,
            _ => return None,
        };
        root.borrow_mut().val = 0;
        
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        println!("{:?}", queue);
        while !queue.is_empty() {

        }

        Some(root)
    }
}


fn main() {
    let mut root = TreeNode::new(312312);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    println!("{:?}", Solution::replace_value_in_tree(Some(Rc::new(RefCell::new(root)))));
}
