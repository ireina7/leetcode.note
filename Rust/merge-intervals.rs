use std::cmp;
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() <= 1 {
            return intervals;
        }
        intervals.sort_by_key(|k| k[0]);
        let mut ans = vec![];
        let (mut i, mut j) = (intervals[0][0], intervals[0][1]);
        for interval in intervals.iter() {
            if interval[0] <= j {
                j = cmp::max(j, interval[1]);
            } else {
                ans.push(vec![i, j]);
                i = interval[0];
                j = interval[1];
            }
        }
        ans.push(vec![i, j]);
        ans
    }
}
