use std::cmp::max;

impl Solution {
    pub fn find_valid_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 1 { return nums }
        let mut result: Vec<i32> = Vec::new();
        let mut left = vec![nums[0]; n];
        for i in 1..n {
            left[i] = max(left[i - 1], nums[i]);
        }
        let mut right = vec![nums[n - 1]; n];
        for i in (0..n - 1).rev() {
            right[i] = max(right[i + 1], nums[i]);
        }
        result.push(nums[0]);
        for i in 1..n - 1 {
            let x = nums[i];
            if x > left[i - 1] || x > right[i + 1] {
                result.push(x);
            }
        }
        result.push(nums[n - 1]);
        result
    }
}
