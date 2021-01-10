impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {

        let n = nums.len();
        (0..n).for_each(|i| {
            let mut x = nums[i] as usize;
            while x > 0 && x <= n && x as i32 != nums[x - 1] {
                nums[i] = nums[x - 1];
                nums[x - 1] = x as i32;
                x = nums[i] as usize;
            }
        });
        (0..n)
            .position(|i| nums[i] != i as i32 + 1)
            .unwrap_or(n) as i32
            + 1
    }
}
