struct Solution;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        
        while !nums.is_empty() {
            let mut res: Vec<i32> = Vec::new();
            res.push(nums.remove(0));
            for i in nums.clone() {
                if i == nums[nums.len() - 1]*nums[nums.len() - 1] {
                    
                }
            }
            nums.pop();
        }
        
        println!("{:?}", nums);
        0
    }
}

fn main() {
    let a = vec![4,3,6,16,8,2,9];
    println!("{}", Solution::longest_square_streak(a));
}
