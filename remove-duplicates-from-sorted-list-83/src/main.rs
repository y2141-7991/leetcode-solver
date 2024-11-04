#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut(); 
        while let Some(currt) = current {
            let mut next_opt = currt.next.take();
            while let Some(next) = next_opt.as_mut() {
                if next.val == currt.val {
                    next_opt = next.next.take();
                }
                else {
                    currt.next = next_opt;
                    break
                }
            }
            current = currt.next.as_mut();        
        }
        head
    }
}

fn main() {
    let example = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: None
            }))
        }))
    };
    println!("{:?}", Solution::delete_duplicates(Some(Box::new(example))));
}
