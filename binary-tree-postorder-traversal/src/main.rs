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

fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if let Some(node) = root {
        let node = node.borrow();
        let mut ans = self::postorder_traversal(node.left.clone());
        ans.extend(self::postorder_traversal(node.right.clone()));
        ans.push(node.val);
        ans.sort();
        ans.reverse();
        ans
    } else {
        vec![]
    }
}

fn main() {
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    println!(
        "{:?}",
        postorder_traversal(Some(Rc::new(RefCell::new(root))))
    )
}
