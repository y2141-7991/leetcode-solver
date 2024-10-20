struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let a: Vec<char> = Self::helpers(n).chars().collect();
        a[(k-1) as usize]
        
    }

    fn helpers(n: i32) -> String {
        if n == 1 {
            return String::from("0");
        }
        let invert_str = Self::invert(Self::helpers(n-1));
        let revert_str = Self::revert(invert_str);
        Self::helpers(n-1) + &String::from("1") + &revert_str
    }

    fn revert(string_input: String) -> String {
        string_input.chars().rev().collect::<String>()
    }

    fn invert(string_input: String) -> String {
        string_input.chars().map(|x| match x {
            '0' => '1',
            '1' => '0',
            _ => x
            
        }).collect::<String>()
    }
}

fn main() {
    let n = 3;
    let k = 1;
    println!("{:?}", Solution::find_kth_bit(n, k))
}
