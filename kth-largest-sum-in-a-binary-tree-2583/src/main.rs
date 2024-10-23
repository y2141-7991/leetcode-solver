use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Reverse;
use std::collections::{VecDeque, BinaryHeap};

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

    pub fn kth_largest_level_sum1(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let root = match root {
            Some(node) => node,
            None => {return -1},
        };
        let mut queue = VecDeque::new();
        queue.push_back(root);

        let mut heap = BinaryHeap::new();

        while !queue.is_empty() {
            let mut level_sum: i64 = 0;
            let length = queue.len();
            for _ in 0..length {
                if let Some(node) = queue.pop_front() {
                    let node = node.borrow();
                    level_sum += node.val as i64;
                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    }
                }
            }
            heap.push(Reverse(level_sum));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        if heap.len() < k as usize {
            return -1;
        }
        let result = match heap.pop() {
            Some(top) => {
                top.0
            },
            None => {
                -1
            }
        };
        result
    }
}

fn main() {
    let mut root = TreeNode::new(312312);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    println!("{:?}", Solution::kth_largest_level_sum(Some(Rc::new(RefCell::new(root))), 1));
}
