struct Solution;

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        println!("{}", 0 ^ 1 ^ 1 ^ 3 ^ 0);
        ans
    }
}

fn main() {
    let a = vec![0,1,1,3];
    let b = 2;
    println!("{:?}", Solution::get_maximum_xor(a, b));
}
