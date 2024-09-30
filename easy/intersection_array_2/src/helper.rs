pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<i32> = Vec::new();

        for num in nums1 {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }

        for num in nums2 {
            if let Some(count) = map.get_mut(&num) {
                if *count > 0 {
                    result.push(num);
                    *count -= 1;
                }
            }
        }

        return result;
    }
}