impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        fn walk<'a, I: Iterator<Item=&'a i32>>(iter: I) -> (i32, i32) {
            iter.fold((0, 0), |(a, b), n| { (b, b.max(a + n)) })
        }
        let (a, b) = walk(nums.iter().take(nums.len() - 1));
        let (c, d) = walk(nums.iter().skip(1));
        a.max(b).max(c).max(d)
    }
}
