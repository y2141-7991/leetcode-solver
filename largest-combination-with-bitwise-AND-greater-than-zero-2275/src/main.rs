struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut largest = 0i32;
        for i in 0..32 {
            let mut count = 0;
            for &c in candidates.iter() {
                count += (c >> i) & 1;
            }
            largest = largest.max(count);
        }

        largest
    }
}

fn main() {
    let candidates = vec![16,17,71,62,12,24,14];
    println!("{}", Solution::largest_combination(candidates));
}
