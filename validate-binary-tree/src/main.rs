use std::cell::RefCell;
use std::rc::Rc;

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

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn helper(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match node {
            None => true,
            Some(ref node) => {
                let val = node.borrow().val;
                if val as i64 <= min || val as i64 >= max {
                    return false;
                }

                helper(node.borrow().left.clone(), min, val as i64)
                    && helper(node.borrow().right.clone(), val as i64, max)
            }
        }
    }
    let min: i64 = -2147483649;
    let max: i64 = 2147483648;

    helper(root, min, max)
}

fn main() {
    let a: u32 = 2147483648 + 1;
    println!("{}", a);
    let root = TreeNode::new(2147483647);
    // root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // root.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    println!("{}", is_valid_bst(Some(Rc::new(RefCell::new(root)))))
}
