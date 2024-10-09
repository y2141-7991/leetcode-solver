struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut open: i32 = 0;
        let mut ans: i32 = 0;
        for i in s.chars().into_iter() {
            if i == '(' {
                open += 1;
            } else if open > 0 {
                open -= 1;
            } else {
                ans += 1;
            }
        }

        return ans + open;
    }
}

fn main() {
    let a = String::from("()))((");
    println!("{}", Solution::min_add_to_make_valid(a))
}
