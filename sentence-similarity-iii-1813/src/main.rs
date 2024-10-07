struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let mut split_s1: Vec<&str> = sentence1.split_whitespace().into_iter().collect();
        let mut split_s2: Vec<&str> = sentence2.split_whitespace().into_iter().collect();
        if split_s1.len() > split_s2.len() {
            std::mem::swap(&mut split_s1, &mut split_s2);
        }
        let mut end_s1 = (split_s1.len() - 1) as isize;
        let mut end_s2 = (split_s2.len() - 1) as isize;
        let mut start = 0;
        
        while start < split_s1.len() && split_s1[start] == split_s2[start] {
            start += 1;
        }
        while end_s1 >= 0 && split_s1[end_s1 as usize] == split_s2[end_s2 as usize] {
            end_s1 -= 1;
            end_s2 -= 1;
        }
        end_s1 < start as isize
    }
}

fn main() {
    let s1 = "Ogn WtWj HneS".to_string();
    let s2 = "Ogn WtWj HneS".to_string();
    println!("{}", Solution::are_sentences_similar(s1, s2));
}
