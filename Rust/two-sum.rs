impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i, j].into_iter().map(|x| x as i32).collect();
                }
            }
        }
        unreachable!()
    }
}
