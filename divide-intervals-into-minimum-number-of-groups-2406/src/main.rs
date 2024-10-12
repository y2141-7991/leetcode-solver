struct Solution;

impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut start_times: Vec<i32> = Vec::new();
        let mut end_times: Vec<i32> = Vec::new();
        for i in intervals.clone() {
            start_times.push(i[0]);
            end_times.push(i[1]);
        }
        start_times.sort();
        end_times.sort();

        let (mut end_ptr, mut group_count) = (0, 0);

        for s in start_times {
            if s > end_times[end_ptr] {
                end_ptr += 1;
            } else {
                group_count += 1;
            }
        }
        group_count
    }
}

fn main() {
    let a = vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]];
    println!("{}", Solution::min_groups(a));
}
