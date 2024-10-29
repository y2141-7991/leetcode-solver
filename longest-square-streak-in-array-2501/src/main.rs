struct Solution;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        const MAX_VALUE: i64 = 100_000;
        let mut seens = vec![false ; MAX_VALUE as usize + 1];
        for &num in nums.iter() {
            seens[num as usize] = true;
        }
        let mut longest_streak = 1;
        for num in nums.into_iter() {
            let mut current = num as i64;
            let mut streak = 1;
            while current * current < MAX_VALUE {
                let square = current * current;
                if seens[square as usize] {
                    streak += 1;
                    current = square;
                }
                else {
                    break
                }
            }
            
            longest_streak = longest_streak.max(streak);
        }
        if longest_streak < 2 {
            -1
        } else {
            longest_streak
        }
    }
}

fn main() {
    let a = vec![4,3,6,16,8,2,9,81];
    println!("{}", Solution::longest_square_streak(a));

}
