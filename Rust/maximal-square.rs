/// Naive dynamic programming
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty(){
            return 0
        }
        let mut memo: Vec<Vec<i32>> = vec![vec![0; matrix[0].len()+1]; matrix.len()+1];
        let mut max = 0;
        for i in 1..=matrix.len(){
            for j in 1..=matrix[0].len() {

                if matrix[i-1][j-1] == '1' {
                    memo[i][j] = 1 + std::cmp::min (
                        memo[i-1][j-1],
                        std::cmp::min(memo[i-1][j], memo[i][j-1])
                    );
                    max = std::cmp::max(max, memo[i][j]);
                }
            }
        }
        max * max
    }
}


/// Dynamic programming with less memory usage.
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty(){
            return 0
        }
        let mut memo: Vec<Vec<i32>> = vec![vec![0; matrix[0].len() + 1]; 2];
        let mut max = 0;
        for i in 1..=matrix.len(){
            for j in 1..=matrix[0].len(){

                memo[1][j] = if matrix[i-1][j-1] == '1' {
                    1 + std::cmp::min (
                        memo[0][j-1],
                        std::cmp::min(memo[0][j], memo[1][j-1])
                    )
                } else { 0 };
                max = std::cmp::max(max, memo[1][j]);
            }
            memo[0] = memo[1].clone();
        }
        max * max
    }
}
