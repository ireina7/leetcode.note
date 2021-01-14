//Naive recursion
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        fn search_max(ns: &[i32]) -> i32 {
            if ns.is_empty() {
                return 1;
            }
            let rest = if ns[0] == 0 {
                0
            } else if ns[0] < 0 {
                search_min(&ns[1..])
            } else {
                search_max(&ns[1..])
            };
            (ns[0] * rest).max(ns[0])
        }
        fn search_min(ns: &[i32]) -> i32 {
            if ns.is_empty() {
                return 1;
            }
            let rest = if ns[0] == 0 {
                0
            } else if ns[0] < 0 {
                search_max(&ns[1..])
            } else {
                search_min(&ns[1..])
            };
            (ns[0] * rest).min(ns[0])
        }
        (0..nums.len()).map(|i| search_max(&nums[i..])).max().unwrap()
    }
}

//Naive memorization
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        fn search_max<'a>(
            ns: &'a [i32],
            memo_min: &mut HashMap<&'a [i32], i32>,
            memo_max: &mut HashMap<&'a [i32], i32>
        ) -> i32 {
            if let Some(&i) = memo_max.get(ns) {
                return i;
            }
            if ns.is_empty() {
                memo_max.insert(ns, 1);
                return 1;
            }
            let rest = if ns[0] == 0 {
                0
            } else if ns[0] < 0 {
                search_min(&ns[1..], memo_min, memo_max)
            } else {
                search_max(&ns[1..], memo_min, memo_max)
            };
            let ans = (ns[0] * rest).max(ns[0]);
            memo_max.insert(ns, ans);
            ans
        }
        fn search_min<'a>(
            ns: &'a [i32],
            memo_min: &mut HashMap<&'a [i32], i32>,
            memo_max: &mut HashMap<&'a [i32], i32>
        ) -> i32 {
            if let Some(&i) = memo_min.get(ns) {
                return i;
            }
            if ns.is_empty() {
                memo_min.insert(ns, 1);
                return 1;
            }
            let rest = if ns[0] == 0 {
                0
            } else if ns[0] < 0 {
                search_max(&ns[1..], memo_min, memo_max)
            } else {
                search_min(&ns[1..], memo_min, memo_max)
            };
            let ans = (ns[0] * rest).min(ns[0]);
            memo_min.insert(ns, ans);
            ans
        }
        let mut memo_min = HashMap::new();
        let mut memo_max = HashMap::new();
        (0..nums.len())
            .map(|i| search_max(&nums[i..], &mut memo_min, &mut memo_max))
            .max()
            .unwrap()
    }
}


//Dynamic programming
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut memo_max = vec![1; n + 1];
        let mut memo_min = vec![1; n + 1];
        for i in (0..n).rev() {
            memo_max[i] = {
                let rest = if nums[i] == 0 {
                    0
                } else if nums[i] < 0 {
                    memo_min[i + 1]
                } else {
                    memo_max[i + 1]
                };
                (nums[i] * rest).max(nums[i])
            };
            memo_min[i] = {
                let rest = if nums[i] == 0 {
                    0
                } else if nums[i] < 0 {
                    memo_max[i + 1]
                } else {
                    memo_min[i + 1]
                };
                (nums[i] * rest).min(nums[i])
            };
        }
        *(&memo_max[..n]).into_iter().max().unwrap()
    }
}

//Dynamic programming save memory: keep optimization!
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut memo_max = 1;
        let mut memo_min = 1;
        let mut max = nums[0];
        for i in (0..n).rev() {
            let curr_max = {
                let rest = if nums[i] == 0 {
                    0
                } else if nums[i] < 0 {
                    memo_min
                } else {
                    memo_max
                };
                (nums[i] * rest).max(nums[i])
            };
            let curr_min = {
                let rest = if nums[i] == 0 {
                    0
                } else if nums[i] < 0 {
                    memo_max
                } else {
                    memo_min
                };
                (nums[i] * rest).min(nums[i])
            };
            memo_max = curr_max;
            memo_min = curr_min;
            max = max.max(memo_max);
        }
        max
    }
}
