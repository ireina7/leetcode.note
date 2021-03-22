impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {
            return;
        }
        fn set_row(i: usize, m: &mut Vec<Vec<i32>>) {
            if m.len() < i + 1 {
                return;
            }
            for j in 0..m[0].len() {
                m[i][j] = 0;
            }
        }
        fn set_col(j: usize, m: &mut Vec<Vec<i32>>) {
            if m.is_empty() || m[0].len() < j + 1 {
                return;
            }
            for i in 0..m.len() {
                m[i][j] = 0;
            }
        }
        use std::collections::HashSet;
        let mut memo_row = HashSet::new();
        let mut memo_col = HashSet::new();
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    memo_row.insert(i);
                    memo_col.insert(j);
                }
            }
        }
        for &i in &memo_row {
            set_row(i, matrix);
        }
        for &j in &memo_col {
            set_col(j, matrix);
        }
    }
}
