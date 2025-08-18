impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        (0..nums.len()).find(|&x| (x % 10) as i32 == nums[x]).map(|x| x as i32).unwrap_or(-1)
    }
}
