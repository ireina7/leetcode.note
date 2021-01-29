impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.iter().fold((0, prices[0]), |(max, min), &p| {
            (max.max(p - min), min.min(p))
        }).0
    }
}
