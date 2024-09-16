struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums: Vec<i32> = nums1;
        nums.extend(nums2);
        nums.sort_by(|a, b| a.cmp(b));
        Self::mean(&nums)
    }
    fn mean(nums: &Vec<i32>) -> f64 {
        let mid = (nums.len() / 2) as f64;
        match nums.len() % 2 {
            0 => {
                let total = nums[mid as usize] + nums[(mid - 1.0) as usize];
                let mean_total = (total as f64) / 2.0;
                return mean_total;
            }
            _ => {
                let num = (mid.round() as i32) as usize;
                return nums[num] as f64;
            }
        }
    }
}

fn main() {
    let a = Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 4]);
}
