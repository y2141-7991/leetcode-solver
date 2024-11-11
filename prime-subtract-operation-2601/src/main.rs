struct Solution;

impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let primes: Vec<_> = (2..*nums.iter().max().unwrap()).filter(|&i| (2..i).all(|j| i % j > 0)).collect();
        let mut prev = 0;

        for i in nums {
            let mut curr = i;
            for &j in primes.iter().rev() {
                if curr > j && curr - j > prev {
                    curr -= j;
                    break;
                }
            }
            if curr <= prev {
                return false;
            }
            prev = curr;
        }
        true
    }
}

fn main() {
    let a = vec![4,9,6,10];
    println!("{}", Solution::prime_sub_operation(a));
}
