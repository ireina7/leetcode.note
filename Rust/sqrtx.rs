//Version 1: naive O(n)
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        (1..).find(|&i| i > x / i).unwrap() - 1
    }
}

//Version 2: Binary search
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }
        let mut i = 0;
        let mut j = x;
        let mut k = (i + j) / 2;
        loop {
            if k <= x / k && (k + 1) > x / (k + 1) {
                return k;
            } else if k > x / k {
                j = k;
            } else if k < x / k {
                i = k;
            }
            k = (i + j) / 2;
        }
    }
}

//Version 3: Newton's method (finding fixed point)
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let x = x as usize;
        let mut n = x;
        while n > x / n {
            n = (n + x / n) / 2;
        }
        n as i32
    }
}
