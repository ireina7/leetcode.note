impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut ans = 1.0;
        let less_0 = n < 0;
        let n = n.abs();
        if n == 0 {
            return 1.0;
        }
        let a = Self::my_pow(x, n / 2);
        ans = a * a * (if n % 2 == 0 {1.0} else {x});

        let ans = if less_0 { 1.0/ans } else { ans };
        if ans.is_infinite() { 0.0 } else { ans }
    }
}
