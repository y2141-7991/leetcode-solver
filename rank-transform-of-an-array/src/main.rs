struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut b: Vec<i32> = Vec::new();
        let mut hash_rank_array: HashMap<i32, i32> = HashMap::new();

        for i in arr.clone() {
            if !b.contains(&i) {
                b.push(i);
            }
        }

        b.sort();
        for (m, n) in b.iter().enumerate() {
            hash_rank_array.entry(*n).or_insert(m as i32 + 1);
        }
        let mut res: Vec<i32> = Vec::new();

        for i in arr.clone() {
            if let Some(o) = hash_rank_array.get(&i) {
                res.push(*o);
            }
        } 

        res
    }
}

fn main() {
    let a = vec![1,1,1,1];
    println!("{:?}", Solution::array_rank_transform(a));
}
