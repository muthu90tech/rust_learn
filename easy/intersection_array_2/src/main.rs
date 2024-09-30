mod helper;

fn main() {
    let nums = vec![1, 2, 2, 3];
    let nums2 = vec![2, 2];
    let result = helper::Solution::intersect(nums, nums2);
    println!("result : {:?}", result);
}