impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize;
        let mut ans = vec![vec![1]; n];
        for i in 1..n {
            for j in 1..i {
                let sum = ans[i-1][j-1] + ans[i-1][j];
                ans[i].push(sum);
            }
            ans[i].push(1);
        }
        ans
    }
}
