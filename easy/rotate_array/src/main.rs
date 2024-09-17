mod helper;

fn main() {
    let mut nums = vec![-1];
    helper::Solution::rotate(&mut nums, 2);
    println!("modified: {:?}", nums);
}