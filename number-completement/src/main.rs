fn main() {
    let mut result = String::new();
    let n: i32 = 1;
    let bin_num = format!("{:b}", n);
    for i in bin_num.chars() {
        if i == '0' {
            result += "1"
        } else {
            result += "0"
        }
    }
    let number = i32::from_str_radix(&result, 2).unwrap();
    println!("{}", number)
}
