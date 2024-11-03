struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let gb: Vec<u8> = goal.bytes().into_iter().collect();
        let mut vb: Vec<u8> = s.bytes().into_iter().collect();
        let mut b= vb.len() as i32;

        while b > 0 {
            let a = vb.remove(0);
            vb.push(a);
            if vb == gb {
                return true;
            }
            b -= 1;
        }
        false
    }
}

fn main() {
    let a = String::from("abcde");
    let b = String::from("cdeab");
    println!("{}", Solution::rotate_string(a, b));
}
