mod helper;

fn main() {
    let nums = vec![1, 1, 2, 2, 3];
    let result = helper::Solution::contains_duplicates(nums);
    println!("Result: {}", result);
}