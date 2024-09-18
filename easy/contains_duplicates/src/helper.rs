use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicates(nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return false;
        }
        
        let mut set: HashSet<i32> = HashSet::new();
        for elem in nums {
            if !set.insert(elem) {
                return true;
            }
        }
        
        return false;
    }
}