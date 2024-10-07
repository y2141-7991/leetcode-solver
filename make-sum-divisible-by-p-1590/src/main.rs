struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let i64_nums: Vec<i64> = nums.clone().into_iter().map(|a: i32| a as i64).collect();

        let i64_sum_nums: i64 = i64_nums.iter().sum();

        let r = i64_sum_nums % p as i64;
        if r == 0 {
            return 0;
        }

        let mut prefix_mod: HashMap<i64, i64> = HashMap::new();
        prefix_mod.insert(0, -1);
        let mut prefix_sum: i64 = 0;
        let mut min_length = nums.len() as i64;

        for (n, m) in nums.iter().enumerate() {
            prefix_sum += *m as i64;
            let current_mod = prefix_sum % p as i64;
            let target_mod = (current_mod as i64 - r + p as i64) % p as i64;

            if prefix_mod.contains_key(&target_mod) {
                let a = prefix_mod.get(&target_mod).unwrap();
                min_length = vec![min_length, (n as i64 - a)].into_iter().min().unwrap();
            }

            prefix_mod.insert(current_mod, n as i64);
        }
        println!("{:?}", min_length);
        if min_length < nums.len() as i64 {
            return min_length as i32;
        } else {
            -1
        }
    }
}

fn main() {
    let nums = vec![6, 3, 5, 2];
    let p = 9;
    println!("{:?}", Solution::min_subarray(nums, p));
}
