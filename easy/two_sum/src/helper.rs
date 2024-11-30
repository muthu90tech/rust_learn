use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<i32> = Vec::new();

        for i in 0..nums.len() {

            if let Some(value) = map.get_mut(&(target-nums[i])) {
                result.push(*value);
                result.push(i.try_into().unwrap());
                return result;
            }
            map.insert(nums[i], i.try_into().unwrap());
        }

        return result;
    }
}