struct Solution;

impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        let mut robot = robot;
        let mut factory = factory;
        robot.sort_unstable();
        factory.sort_unstable();

        let m = robot.len();
        let n = factory.len();

        let mut dp = vec![vec![0i64; n + 1]; m + 1];
        for i in 0..m {
            dp[i][n] = i64::MAX;
        }
        for j in (0..n).rev() {
            let mut prefix = 0i64;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((m, 0i64));
            for i in (0..m).rev() {
                prefix += (robot[i] - factory[j][0]).abs() as i64;
                while !queue.is_empty() && queue[0].0 > i + factory[j][1] as usize{
                    queue.pop_front();
                }
                while !queue.is_empty() && queue.back().unwrap().1 >= dp[i][j+1] - prefix {
                    queue.pop_back();
                }
                queue.push_back((i, dp[i][j+1] - prefix));
                dp[i][j] = queue[0].1 + prefix;
            }

        }
        // println!("{:?}", dp);
        dp[0][0]
    }
}

fn main() {
    let robot = vec![0,4,6];
    let factory = vec![vec![2,2],vec![6,2]];
    println!("{}", Solution::minimum_total_distance(robot, factory))
}
