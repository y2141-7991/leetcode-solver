use core::num;

struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        for n in 0..nums.len() {
            let mut cloned_nums = nums.clone();
            cloned_nums.remove(n);

            let sum_cloned_nums: i32 = cloned_nums.iter().sum();
            if sum_cloned_nums % p == 0 {
               return (nums.len() - cloned_nums.len()) as i32
            }

            println!("{:?}", cloned_nums);
            Self::min_subarray(cloned_nums, p);
        }
        let sum_nums: i32 = nums.iter().sum();
        if sum_nums % p == 0 {
            return 0
        } else {
            -1
        }
    }
    fn helpers(nums: Vec<i32>, p: i32, n: usize) -> Vec<i32> {
        let mut cloned_nums = nums.clone();
        cloned_nums.remove(n);
        cloned_nums
    }

    pub fn min_subarray1(nums: Vec<i32>, p: i32) -> i32 {
        let sum_nums: i32 = nums.iter().sum();
        if sum_nums % p == 0 {
            return 0
        }

        for n in 0..nums.len() {
            let a = Self::helpers(nums.clone(), p, n);
            let das:i32 = a.iter().sum();
            if das % p == 0 && a.len() > 0{
                println!("{}", (nums.len() - a.len()) as i32);
            }
            Self::min_subarray1(a, p);
            
        }
        
        -1
    }
}

fn main() {
    let nums = vec![6,3,5,2];
    let p = 9;
    println!("{:?}",Solution::min_subarray1(nums, p));
}
