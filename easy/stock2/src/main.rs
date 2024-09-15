mod helper;

fn main() {
    let prices = vec![1,2,3,4,5];
    let result = helper::Solution::max_profit(prices);
    println!("Result: {}", result);
}