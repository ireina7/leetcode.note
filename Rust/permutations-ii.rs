impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {

        fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            if nums.len() <= 1 {
                return vec![nums]
            }
            let mut prev: Option<i32> = None;
            let mut res = Vec::new();
            for (i, &num) in nums.iter().enumerate() {
                if prev.is_some() && prev.unwrap() == num { continue }
                else { prev = Some(num) }
                let mut sub = nums.clone();
                sub.remove(i);
                let mut permutations: Vec<Vec<i32>> =
                    permute(sub).into_iter().map(|x| {
                    let mut x = x; x.push(num); x
                }).collect();
                res.append(&mut permutations);
            }
            res
        }
        let mut nums = nums;
        nums.sort_unstable();
        permute(nums)
    }
}
