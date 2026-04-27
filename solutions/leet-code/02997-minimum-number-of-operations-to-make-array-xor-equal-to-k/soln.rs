impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        (nums.clone().into_iter().skip(1).fold(nums[0],|a,c| a^c)^k).count_ones() as i32
    }
}
