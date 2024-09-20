mod helper;

fn main() {
    let nums = vec![1, 1, 2, 3];
    let result = helper::Solution::single_number(nums);
    println!("Result: {}", result);
}