struct Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut cmp: Vec<char> = Vec::new();
        let word: Vec<u8> = word.bytes().into_iter().collect();
        let mut idx = 0;
        while idx < word.len() {
            let mut count = 0;
            let current_char = word[idx];
            while idx < word.len() && word[idx] == current_char && count < 9 {
                count += 1;
                idx += 1;
            }
            cmp.push((count + b'0') as char);
            cmp.push(current_char as char);
        }
        cmp.into_iter().collect()
    }
}

fn main() {
    let a = String::from("aaabbbcde");
    println!("{}", Solution::compressed_string(a));
}
