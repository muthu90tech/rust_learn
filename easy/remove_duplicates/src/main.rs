mod helper;

fn main() {
    let mut nums = vec![1, 1, 2, 2, 3];
    let result = helper::Solution::remove_duplicates(&mut nums);
    println!("Result: {}", result);
    println!("modified: {:?}", nums);
}