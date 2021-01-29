//Naive recursion
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        if board.is_empty() {
            return;
        }
        fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, c: char)
        -> bool {
            for i in 0..9 {
                if (board[row][i] != '.' && board[row][i] == c ||
                    board[i][col] != '.' && board[i][col] == c) {
                    return false;
                }
                let block = board
                    [(3 * (row as i32 / 3) + i as i32 / 3) as usize]
                    [(3 * (col as i32 / 3) + i as i32 % 3) as usize];
                if block != '.' && block == c { return false; } // check 3 * 3 block;
            }
            true
        }
        fn search(board: &mut Vec<Vec<char>>) -> bool {
            let h = board.len();
            let w = board[0].len();
            for i in 0..h {
                for j in 0..w {
                    if board[i][j] != '.' {
                        continue;
                    }
                    for n in "123456789".chars() {
                        if !is_valid(board, i, j, n) {
                            continue;
                        }
                        board[i][j] = n;
                        if search(board) {
                            return true;
                        }
                        board[i][j] = '.';
                    }
                    return false;
                }
            }
            true
        }
        search(board);
    }
}
