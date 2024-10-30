
struct Solution;

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 3 {
            return 0;
        }
        let mut res = vec![];
        for i in 1..n - 1 {
            if nums[i] >= nums[i-1] && nums[i] >= nums[i+1] {
                let max_val = nums[i];
                let idx_max_val = i;
                
                let mut dp = vec![false; n];
                dp[idx_max_val] = true;
                let dp2 = Self::mapping2(dp.clone(), max_val, idx_max_val, nums.clone());
                let dp1 = Self::mapping1(dp.clone(), max_val, idx_max_val, nums.clone());
                let res1 = dp1.iter().filter(|&x| *x == false).count();
                let res2 = dp2.iter().filter(|&x| *x == false).count();
                println!("{}, {}, {}", idx_max_val, max_val, res1.min(res2));
                println!("{:?}", dp1);
                println!("{:?}", dp2);
                res.push(res1.min(res2));
            }
        }
        res.into_iter().min().unwrap() as i32
    }

    fn mapping2(mut dp: Vec<bool>, max_val: i32, idx_max_val: usize, nums: Vec<i32>) -> Vec<bool> {
        let n = nums.len();
        let mut cmp_num = max_val;
        for i in (0..idx_max_val).rev() {
            if nums[i] < cmp_num as i32 {
                dp[i] = true;
                cmp_num = nums[i];
            }
            
        }
        cmp_num = max_val;
        for i in idx_max_val+1..n {
            if nums[i] < cmp_num as i32 {
                dp[i] = true;
                cmp_num = nums[i];
            }
            
        }
        dp
    }
    fn mapping1(mut dp: Vec<bool>, max_val: i32, idx_max_val: usize, nums: Vec<i32>) -> Vec<bool> {
        let n = nums.len();
        let mut max_num = 0;
        for i in 0..idx_max_val {
            if nums[i] < max_val as i32 && nums[i] > max_num {
                // println!("{}, {}, {}", nums[i], nums[i+1], max_val);
                dp[i] = true;
                max_num = nums[i]
            }
            
        }

        let mut min_num = 0;
        for i in (idx_max_val+1..n).rev() {
            if nums[i] < max_val as i32 && nums[i] > min_num {
                dp[i] = true;
                min_num = nums[i];
            }
        }
        dp
    }
}

fn main() {
    let a = vec![4,5,13,17,1,7,6,11,2,8,10,15,3,9,12,14,16];
    println!("{:?}", Solution::minimum_mountain_removals(a));
}
