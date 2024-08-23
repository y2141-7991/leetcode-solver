fn find_an_empty_cell(board: &mut Vec<Vec<char>>) -> Option<(i32, i32)> {
    for m in 0..9 {
        for n in 0..9 {
            if board[m][n] == '.' {
                return Some((m as i32, n as i32))
            }
        }
    }
    None
}

fn solve(board: &mut Vec<Vec<char>>) -> bool {
    let empty_cell = find_an_empty_cell(board);
    if let Some((row, col)) = empty_cell {
        for i in 1..=9 {
            if is_valid_value(row, col, i, board) {
                board[row as usize][col as usize] = i.to_string().parse::<char>().unwrap_or_default();
                if solve(board) {
                    return true;
                }
            }
        }
        board[row as usize][col as usize] = '.'
    } else {
        return true;
    }
    return false
}

fn is_valid_value(row: i32, col: i32, value: i32, board: &mut Vec<Vec<char>>) -> bool {
    
    for n in 0..9 {
        if board[row as usize][n].to_string().parse::<i32>().unwrap_or_default() == value {
            return false
        }
    }
    for m in 0..9 {
        if board[m][col as usize].to_string().parse::<i32>().unwrap_or_default() == value {
            return false
        }
    }
    let table_row = row / 3 * 3;
    let table_col = col / 3 * 3;
    for r in table_row..table_row + 3 {
        for c in table_col..table_col + 3 {
            if board[r as usize][c as usize].to_string().parse::<i32>().unwrap_or_default() == value {
                return false
            }
        }
    }
    if row == 0 {
        println!("{}, {}, {}", row, col, value);
    }
    true
}

fn main() {
    let mut board: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let a = solve(&mut board);
    println!("{:?}", board);
}
