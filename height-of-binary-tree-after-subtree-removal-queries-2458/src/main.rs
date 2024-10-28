use std::cell::RefCell;
use std::rc::Rc;
use std::vec;

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
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let root = match root {
            Some(node) => node,
            None => return res
        };
        let mut total = 0;
        for i in queries {
            let a: i32 = Self::helpers(root.borrow().left.clone(), i, total);
            let b: i32 = Self::helpers(root.borrow().right.clone(), i, total);
            println!("{:?} {:?}, {}", a, b, total)
        }
        res   
    }

    fn helpers(root: Option<Rc<RefCell<TreeNode>>>, n: i32, mut total: i32) -> i32 {
        
        let node = match root {
            Some(node) => node,
            None => return 0,
        };
        if node.borrow().val == n {
            return 0;
        }
        if !node.borrow().left.is_none() || !node.borrow().right.is_none() {
            total += 1
        }
        let left_side = Self::helpers(node.borrow().left.clone(), n, total);

        let right_side = Self::helpers(node.borrow().right.clone(), n, total);
        vec![left_side, right_side].into_iter().max().unwrap()
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
    println!("{:?}", Solution::tree_queries(root, vec![1]));
}
