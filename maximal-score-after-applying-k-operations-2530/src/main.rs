struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from(nums);
        let mut result: i64 = 0;
        let mut v = 0;
        println!("{:?}", heap);
        for i in 0..k {
            v = heap.pop().unwrap();
            println!("{}", v);
            println!("{:?}", heap);
            result += v as i64;
            heap.push((v+2)/3);
            println!("{:?}", heap);
            
        }
        result
    }
}

fn main() {
    let a = vec![1,10,3,3,3];
    let b = 3;
    println!("{}", Solution::max_kelements(a, b));
}
