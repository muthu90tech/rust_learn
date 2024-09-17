pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
        k %= nums.len() as i32;
        Self::reverse(nums, 0, (nums.len() - 1) as i32);
        Self::reverse(nums, 0, k - 1);
        Self::reverse(nums, k, (nums.len() - 1) as i32);
    }
    pub fn reverse(nums: &mut Vec<i32>, mut start: i32, mut end: i32) {
        while start < end {
            let temp = nums[start as usize];
            nums[start as usize] = nums[end as usize];
            nums[end as usize] = temp;
            start += 1;
            end -= 1;
        }
    }
}