struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let mut s1_count = vec![0; 26];
        let mut s2_count = vec![0; 26];
        for i in 0..s1.len() {
            s1_count[(s1.chars().nth(i).unwrap() as i32 - 97) as usize] += 1;
            s2_count[(s2.chars().nth(i).unwrap() as i32 - 97) as usize] += 1;
        }

        for n in 0..(s2.len() - s1.len()) {
            if s1_count == s2_count {
                return true;
            }
            s2_count[(s2.chars().nth(n).unwrap() as i32 - 97) as usize] -= 1;
            s2_count[(s2.chars().nth(n + s1.len()).unwrap() as i32 - 97) as usize] += 1;
        }

        return s1_count == s2_count;
    }
}

fn main() {
    let s1 = "ab".to_string();
    let s2 = "eidbaooo".to_string();
    println!("{}", Solution::check_inclusion(s1, s2));
}
