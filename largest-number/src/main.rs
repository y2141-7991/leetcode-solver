
struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut string_num: Vec<_> = nums.into_iter().map(|n| n.to_string()).collect();
        string_num.sort_by(|a, b| {
            let a_first_digit = a.chars().nth(0).unwrap();
            let b_first_digit = b.chars().nth(0).unwrap();
            b_first_digit.cmp(&a_first_digit)
        });
        string_num.sort_by(|a, b| {
            let s1 = format!("{}{}", a, b);
            let s2 = format!("{}{}", b, a);
            s2.cmp(&s1)
        });
        if string_num[0] == "0" {
            return "0".to_string();
        }
        string_num.join("")
    }
}

fn main() {
    let nums = vec![3,30,34,5,9];
    let a = Solution::largest_number(nums.clone());
    println!("{:?}", a)
}
