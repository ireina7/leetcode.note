/// Naive sorting
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        nums.sort();
    }
}

/// Counting sort (since colors only: 0, 1, 2)
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        fn counting_sort(xs: &mut [i32]) {
            if xs.is_empty() {
                return;
            }
            let max = xs.iter().max().unwrap_or(&0);
            let mut occurences: Vec<usize> = vec![0; *max as usize + 1];

            for &x in xs.iter() {
                occurences[x as usize] += 1;
            }

            let mut i = 0;
            for (x, &number) in occurences.iter().enumerate() {
                for _ in 0..number {
                    xs[i] = x as i32;
                    i += 1;
                }
            }
        }
        counting_sort(nums);
    }
}
