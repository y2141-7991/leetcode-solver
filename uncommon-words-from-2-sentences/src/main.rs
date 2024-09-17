struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut sentence1: Vec<&str> = s1.as_str().split_whitespace().into_iter().collect();
        let sentence2: Vec<&str> = s2.as_str().split_whitespace().into_iter().collect();
        sentence1.extend(sentence2);
        let mut frequency: HashMap<&str, i32> = HashMap::new();
        for w in sentence1 {
            *frequency.entry(w).or_insert(0) += 1;
        }
        let b: Vec<_> = frequency
            .iter()
            .filter_map(|(k, v)| {
                if *v == 1 {
                    Some((k, v))
                } else {
                    Some((&"", &2))
                }
            })
            .collect();
        let mut returned: Vec<String> = Vec::new();
        for (m, n) in b {
            if *n == 1 {
                returned.push(m.to_string())
            }
        }
        returned
    }
}

fn main() {
    let a = Solution::uncommon_from_sentences(
        "this apple is sweet".to_string(),
        "this apple is sour".to_string(),
    );
}
