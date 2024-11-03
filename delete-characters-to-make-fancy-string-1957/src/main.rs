struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut bytestring: Vec<u8> = s.bytes().collect();
        let mut count = 0;
        let mut p: u8 = 0;
        for i in 0..bytestring.len() {
            if p != bytestring[i] {
                p = bytestring[i];
                count = 0;
            }
            count += 1;
            if count > 2 {
                bytestring[i] = 0;
            }
        }
        bytestring = bytestring
            .into_iter()
            .filter(|&x| x != 0)
            .collect::<Vec<u8>>();
        let bytestring = bytestring
            .into_iter()
            .map(|x| x as char)
            .collect::<Vec<char>>();
        bytestring.into_iter().collect::<String>()
    }
}

fn main() {
    let a = String::from("leeeeeetcode");
    println!("{}", Solution::make_fancy_string(a));
}
