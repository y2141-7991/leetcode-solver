struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut freq: Vec<i32> = vec![0; k as usize];
        for n in arr {
            let r = ((n % k + k) % k) as usize;
            freq[r] += 1;
        }
        if freq[0] % 2 != 0 {
            return false;
        }
        for i in 1..(k / 2 + 1) {
            if freq[i as usize] != freq[(k - i) as usize] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9];
    let k = 5;
    Solution::can_arrange(arr, k);
}
