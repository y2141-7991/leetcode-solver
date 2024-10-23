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
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let nums: Vec<i64>= Vec::new();
        fn helpers(nums: Vec<Option<Rc<RefCell<TreeNode>>>>, mut vec_nums: Vec<i64>) -> Vec<i64> {
            let mut sub_nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
            let mut val: i64 = 0;
            for n in nums {
                if let Some(n) = n {
                    let val_num = n.borrow();
                    val += val_num.val as i64;
                    sub_nodes.extend(vec![val_num.right.clone(), val_num.left.clone()]);
                }

            }
            if val == 0 {
                return vec_nums;
            }
            vec_nums.push(val);
            helpers(sub_nodes, vec_nums) 
        }
        let mut res = helpers(vec![root], nums);
        if res.len() < k as usize{
            return -1;
        }
        res.sort_by(|a, b| b.cmp(a));
        res[(k as usize) - 1]

    }
}

fn main() {
    let mut root = TreeNode::new(312312);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    println!("{:?}", Solution::kth_largest_level_sum(Some(Rc::new(RefCell::new(root))), 1));
}
