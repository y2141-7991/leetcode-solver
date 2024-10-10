
struct Solution;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut min_lst = vec![0; sz];
        let mut max_lst = vec![0; sz];

        min_lst[0] = nums[0];
        for i in 1..sz {
            min_lst[i] = min_lst[i - 1].min(nums[i]);
        }
        max_lst[sz - 1] = nums[sz - 1];
        for i in (0..sz - 1).rev() {
            max_lst[i] = max_lst[i + 1].max(nums[i]);
        }
        let mut left = 0;
        let mut right = 0;

        let mut ret = 0;
        while right < sz {
            if min_lst[left] <= max_lst[right] {
                ret = ret.max((right - left) as i32);
                right += 1;
            } else {
                left += 1;
            }
        }

        ret
    }
}


fn main() {
    let a = vec![9,8,1,0,1,9,4,0,4,1];
    println!("{}", Solution::max_width_ramp(a));
}
