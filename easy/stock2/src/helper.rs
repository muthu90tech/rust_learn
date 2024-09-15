pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let mut profit = 0;
        for i in 0..(prices.len() - 1) {
            if prices[i] < prices[i+1] {
                profit += prices[i+1] - prices[i];
            }
        }
        return profit as i32;
    }
}