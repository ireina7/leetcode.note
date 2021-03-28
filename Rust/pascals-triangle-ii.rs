impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let n = row_index as usize;
        let mut ans = vec![1; n + 1];
        for i in 1..=n {
            let tmp = ans.clone();
            for j in 1..i {
                ans[j] = tmp[j-1] + tmp[j];
            }
            ans[i] = 1;
        }
        ans
    }
}
