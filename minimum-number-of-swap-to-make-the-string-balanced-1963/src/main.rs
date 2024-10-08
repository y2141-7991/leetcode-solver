struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut ans = 0;
        for c in s.chars().into_iter() {
            if c == '[' {
                ans += 1;
            }
            else if ans > 0 {
                ans -= 1
            }
        }
        return (ans + 1) / 2;
    }
}

fn main() {
    let a = String::from("]]][[[");
    println!("{}", Solution::min_swaps(a));
}
