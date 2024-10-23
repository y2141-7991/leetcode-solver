use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Clone)]
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
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = match root {
            Some(node) => node,
            _ => return None,
        };

        let mut queue = VecDeque::new();
        root.borrow_mut().val = 0;
        queue.push_back(root.clone());

        while !queue.is_empty() {
            let sum = queue
                .iter()
                .map(|n| {
                    let n = n.borrow();
                    n.left.as_ref().map_or(0, |n| n.borrow().val)
                        + n.right.as_ref().map_or(0, |n| n.borrow().val)
                })
                .sum::<i32>();
            println!("{}", sum);
            for _ in 0..queue.len() {
                let n = queue.pop_front().unwrap();
                let n = n.borrow_mut();
                let s = sum
                    - n.left.as_ref().map_or(0, |n| n.borrow_mut().val)
                    - n.right.as_ref().map_or(0, |n| n.borrow_mut().val);
                if let Some(left_val) = n.left.clone() {
                    left_val.borrow_mut().val = s;
                    queue.push_back(left_val);
                }
                if let Some(right_val) = n.right.clone() {
                    right_val.borrow_mut().val = s;
                    queue.push_back(right_val);
                }
            }
        }
        Some(root)
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));
    println!("{:?}", Solution::replace_value_in_tree(root));
}
