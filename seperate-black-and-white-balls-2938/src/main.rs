struct Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut swaps: i64 = 0;
        let mut long = 0;
        for i in s.chars().into_iter() {
            if i == '0' {
                swaps += long as i64;
            }
            else {
                long += 1;
            }
        }
        swaps
    }
}

fn main() {
    let a = String::from("01100010");
    println!("{}", Solution::minimum_steps(a));
}
