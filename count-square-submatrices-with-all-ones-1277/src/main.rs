struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut num_squares = 0;
        let n = matrix.len();
        let m = matrix[0].len();

        let mut dp = vec![vec![0; m]; n];
        for i in 0..n {
            dp[i][0] = matrix[i][0];
            num_squares += dp[i][0]
        }

        for j in 1..m {
            dp[0][j] = matrix[0][j];
            num_squares += dp[0][j];
        }
        println!("{:?}, {}", dp, num_squares); 

        for i in 1..n {
            for j in 1..m {
                if matrix[i][j] == 1 {
                    dp[i][j] = 1 + vec![dp[i][j-1], dp[i-1][j], dp[i-1][j-1]].into_iter().min().unwrap();
                }
                num_squares += dp[i][j]
            }
        }

        println!("{:?}, {}", dp, num_squares); 
        num_squares
    }
}

fn main() {
    let a = vec![
        vec![0,1,1,1],
        vec![1,1,1,1],
        vec![0,1,1,1]
      ];
    println!("{:?}", Solution::count_squares(a));
}
