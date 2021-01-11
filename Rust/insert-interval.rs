impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::{min, max};
        let mut less = vec![];
        let mut more = vec![];
        let mut curr = new_interval.clone();

        for range in intervals {
            if range[1] < new_interval[0]{
                less.push(range);
            }
            else if range[0] > new_interval[1]{
                more.push(range);
            }
            else {
                curr = vec![
                    min(range[0], curr[0]),
                    max(range[1], curr[1])
                ];
            }
        }
        less.push(curr.clone());
        less.append(&mut more);
        less
    }
}
