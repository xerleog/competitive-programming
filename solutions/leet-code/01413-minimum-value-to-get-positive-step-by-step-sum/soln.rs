impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        (1 - nums.into_iter().scan(0, |prefix_sum, num| {*prefix_sum += num; Some(*prefix_sum)}).min().unwrap_or(0)).max(1)
    }
}
