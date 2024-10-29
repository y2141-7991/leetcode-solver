struct Solution;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: &[i32] = &[-1, 0, 1];
        let mut queue: Vec<(i32, i32, i32)> = vec![];
        let m = grid.len();
        let n = grid[0].len();

        let mut dp = vec![vec![false; m]; n];
        let mut res = 0;
        for i in 0..m {
            dp[i][0] = true;
            queue.push((i as i32, 0, 0));
        }
        while let Some(top) = queue.pop() {
            let (row, col, count) = (top.0, top.1, top.2);
            res = res.max(count);
            for &dir in DIRS.iter() {
                let (new_row, new_col) = (row + dir, col + 1);
                if new_row >= 0
                    && new_col >= 0
                    && new_row < m as i32
                    && new_col < n as i32
                    && !dp[new_row as usize][new_col as usize]
                    && grid[row as usize][col as usize] < grid[new_row as usize][new_col as usize]
                {
                    queue.push((new_row, new_col, count + 1));
                }
            }
        }

        println!("{:?}", dp);
        0
    }
}

fn main() {
    let a = vec![
        vec![2, 4, 3, 5],
        vec![5, 4, 9, 3],
        vec![3, 4, 2, 11],
        vec![10, 9, 13, 15],
    ];
    println!("{}", Solution::max_moves(a));
}
