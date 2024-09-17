fn main() {
    let decimal = 0.5;
    let mut numerator = decimal as i32;
    let mut denominator = 1;

    while numerator % 10 != 0 {
        numerator *= 10;
        denominator *= 10;
    }

    let gcd = gcd(numerator, denominator);
    let numerator = numerator / gcd;
    let denominator = denominator / gcd;

    println!("0.5 as a fraction: {}/{}", numerator, denominator);
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
