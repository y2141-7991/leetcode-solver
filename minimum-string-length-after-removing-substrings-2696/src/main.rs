struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        if !s.contains("AB") && !s.contains("CD") {
            return s.len() as i32;
        }
        let mut sub = s;
        sub = sub.replace("AB", "");
        sub = sub.replace("CD", "");
        
        Self::min_length(sub)
    }
}

fn main() {
    let a = "ABFCACDB".to_string();
    println!("{}", Solution::min_length(a))
}
