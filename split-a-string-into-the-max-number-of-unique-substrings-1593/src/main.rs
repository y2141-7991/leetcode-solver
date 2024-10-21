struct Solution;

impl Solution {
    pub fn max_unique_split(s: String) {
        let c= s.as_bytes();
        let max_breaks: u32 = (1 << (c.len() - 1)) -1;
        let mut out = 1u32;
        for i in (0..=max_breaks).rev() {
            println!("{}, {}", i.trailing_zeros(), i);
            if i.count_ones() + 1 > out && Self::validate_split(c, i) {
                out = i.count_ones() + 1;
            }
        }
        println!("{}", out);
    }
    fn validate_split(mut s: &[u8], mut b: u32) -> bool {
        let mut substring: Vec<&[u8]> = Vec::new();
        while !s.is_empty() {
            let breakpoint = (b.trailing_zeros() + 1) as usize;
            println!("{:b}, {}", b, breakpoint, );
            b >>= breakpoint;
            println!("{:b}", b);
            let (to_add, rem) = s.split_at(breakpoint.min(s.len()));
            if substring.contains(&to_add) {
                return false;
            }
            substring.push(to_add);
            s = rem;
        }
        true
    }
}


fn main() {
    let a = String::from("ababccc");
    println!("{:?}", Solution::max_unique_split(a));

    let a: u32 = 0;
    println!("{}", a >> 31);
}
