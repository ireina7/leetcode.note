impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {

        fn next(nums: &mut Vec<i32>, s: usize) -> Vec<Vec<i32>> {
            if s >= nums.len() {
                return vec![vec![]];
            }
            (s..nums.len()).flat_map(|i| {
                nums.swap(s, i);
                let mut res = next(nums, s + 1);
                for p in res.iter_mut() {
                    p.insert(0, nums[s]);
                }
                nums.swap(i, s);
                res
            }).collect()
        }
        next(&mut nums, 0)
    }
}
