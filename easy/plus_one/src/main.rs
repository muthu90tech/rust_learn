mod helper;

fn main() {
    let nums = vec![1,2,3];
    let result = helper::Solution::plus_one(nums);
    println!("Result: {:?}", result);
}