//Naive recursion with memorization
use std::collections::HashMap;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn try_it<'a>(nums: &'a[i32], memo: &mut HashMap<&'a[i32], i32>) -> i32 {
            if let Some(&i) = memo.get(nums) {
                return i;
            }
            if nums.is_empty() {
                memo.insert(nums, 0);
                return 0;
            }
            if nums.len() == 1 {
                memo.insert(nums, nums[0]);
                return nums[0];
            }
            let doit = try_it(&nums[2..], memo);
            let dont = try_it(&nums[1..], memo);
            let ans = dont.max(doit + nums[0]);
            memo.insert(nums, ans);
            ans
        }
        let mut memo = HashMap::new();
        try_it(&nums, &mut memo)
    }
}
