pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut carry = 1;

        for dig in digits.iter().rev() {
            let last_dig_sum = dig + carry;
            let value = last_dig_sum % 10;
            carry = last_dig_sum / 10;
            result.insert(0, value);
        }

        if carry != 0 {
            result.insert(0, carry);
        }

        return result;
    }
}