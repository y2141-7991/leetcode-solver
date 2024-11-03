struct Solution;

impl Solution {
    fn lis_length(v: &Vec<i32>) -> Vec<i32> {
        let mut lis = vec![v[0]];
        let mut lis_len = vec![1; v.len()];

        for i in 1..v.len() {
            if v[i] > *lis.last().unwrap() {
                lis.push(v[i]);
            } else {
                let index = lis.partition_point(|&x| x < v[i]);
                lis[index] = v[i];
            }
            lis_len[i] = lis.len() as i32
        }
        lis_len
    }

    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let lis = Self::lis_length(&nums);

        let mut reversed_nums = nums.clone();
        reversed_nums.reverse();

        let mut lds = Self::lis_length(&nums);
        lds.reverse();

        let mut removals = i32::MAX;

        for i in 0..n {
            if lis[i] > 1 && lds[i] > 1 {
                removals = removals.min(n as i32 + 1 - lis[i] - lds[i]);
            }
        }
        removals
    }
}

fn main() {
    let a = vec![4, 5, 13, 17, 1, 7, 6, 11, 2, 8, 10, 15, 3, 9, 12, 14, 16];
    println!("{:?}", Solution::minimum_mountain_removals(a));
}
