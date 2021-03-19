impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let result = dividend.checked_div(divisor);

        match result {
            Some(i) => i,
            None => std::i32::MAX,
        }
    }
}
