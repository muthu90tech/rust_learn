pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        
        let mut index = 0;
        for i in  1..nums.len() {
            if nums[i] != nums[index] {
                index += 1;
                nums[index] = nums[i];
            }
        }

        (index + 1) as i32
    }
}