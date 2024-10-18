struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut max_value = 0;
        let len_nums = nums.len();

        for i in nums.clone() {
            max_value |= i;
        }
        let total_subset = (1 << len_nums) - 1;
        let mut cnt = 1;
        
        for n in 1..total_subset {
            let mut or_sub_set = 0;
            
            for m in 0..len_nums {
                if n & (1 << m) != 0 {
                    or_sub_set |= nums[m];
                }
                
            }
            if or_sub_set == max_value { cnt += 1 }
        }
        cnt
    }
}

fn main() {
    let a = vec![3,2,1,5];
    println!("{:?}", Solution::count_max_or_subsets(a));
}
