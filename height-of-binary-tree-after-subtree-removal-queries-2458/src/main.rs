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
        type D = [(i32, i32); 100001];
        let mut ld = [(-1, -1); 100001];
        let mut vld = ld.clone();
        fn dfs(ld: &mut D, vld: &mut D, n: &Option<Rc<RefCell<TreeNode>>>, lvl: i32) -> i32 {
            let Some(n) = n else { return -1 };
            let mut n = n.borrow_mut();
            let d = 1 + dfs(ld, vld, &n.left, lvl + 1).max(dfs(ld, vld, &n.right, lvl + 1));
            vld[n.val as usize] = (lvl, d);
            let m = &mut ld[lvl as usize];
            if d > m.0 {
                m.1 = m.0;
                m.0 = d
            } else {
                m.1 = m.1.max(d)
            };
            d
        }
        dfs(&mut ld, &mut vld, &root, 0);
        queries
            .iter()
            .map(|&q| {
                let (lvl, d) = vld[q as usize];
                let (d1, d2) = ld[lvl as usize];
                lvl + if d < d1 { d1 } else { d2 }
            })
            .collect()
    }

    pub fn tree_queries1(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        let total = 0;
        for i in queries {
            let a: i32 = Self::helpers(root.clone(), i, total);
            res.push(a);
        }
        res
    }

    fn helpers(root: Option<Rc<RefCell<TreeNode>>>, n: i32, mut total: i32) -> i32 {
        let node = match root {
            Some(node) => node,
            None => return total,
        };
        if node.borrow().val == n {
            return 0;
        }
        if node.borrow().val != n {
            total += 1;
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
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 10,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 10,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
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
    println!("{:?}", Solution::tree_queries(root, vec![9]));
}
