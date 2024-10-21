struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let a: Vec<u8> = Self::helpers1(n);
        a[(k-1) as usize] as char
        
    }
    fn helpers1(n: i32) -> Vec<u8> {
        if n == 1 {
            return vec![48];
        }
        let invert_str = Self::invert1(Self::helpers1(n-1));
        let revert_str = Self::revert1(invert_str);
        let mut res = Self::helpers1(n-1);
        res.push(49);
        res.extend(&revert_str);
        res
    }

    fn revert1(string_input: Vec<u8>) -> Vec<u8> {
        string_input.into_iter().rev().collect::<Vec<u8>>()
        
    }

    fn invert1(string_input: Vec<u8>) -> Vec<u8> {
        string_input.into_iter().map(|x| match x {
            b'0' => b'1',
            b'1' => b'0',
            _ => x
            
        }).collect::<Vec<u8>>()
    }
}

fn main() {
    let n = 3;
    let k = 1;
    println!("{:?}", Solution::find_kth_bit(n, k));

    println!("{:?}", Solution::helpers1(3));

    let mut c = vec![48, 49, 48, 49, 48, 49, 48];
    c.extend(vec!['1' as u8]);
    println!("{:?}", c)
}
