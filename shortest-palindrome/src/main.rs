struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let a: String = s.chars().rev().into_iter().collect();
        for i in 0..s.len() + 1 {
            if s.starts_with(&a.as_str()[i..s.len()]) {
                return a.as_str()[0..i].to_string() + &s;
            }
        }
        s
    }
}

fn main() {
    println!("{:?}", Solution::shortest_palindrome(String::from("abcd")));
}
